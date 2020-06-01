use super::{HitRecord, Ray};
use cgmath::{vec3, InnerSpace, Vector3};
use rand::distributions::Standard;
use rand::prelude::*;

fn rand_in_unit_sphere<T, R>(rng: &mut R) -> Vector3<T>
where
    T: cgmath::BaseFloat,
    Standard: Distribution<T>,
    R: rand::Rng,
{
    loop {
        let x = rng.gen::<T>();
        let y = rng.gen::<T>();
        let z = rng.gen::<T>();
        let one = T::one();
        let p = vec3(x, y, z) * T::from(2.0).unwrap() - vec3(one, one, one);
        if p.magnitude2() < one {
            return p;
        }
    }
}

pub trait Material<T, R> {
    fn scatter(
        &self,
        rng: &mut R,
        r: &Ray<T>,
        rec: &HitRecord<T, R>,
    ) -> Option<(Vector3<T>, Ray<T>)>;
}

pub struct Lambertian<T> {
    albedo: Vector3<T>,
}

impl<T> Lambertian<T> {
    pub fn new(albedo: Vector3<T>) -> Self {
        Self { albedo }
    }
}

impl<T, R> Material<T, R> for Lambertian<T>
where
    T: cgmath::BaseFloat,
    Standard: Distribution<T>,
    R: rand::Rng,
{
    fn scatter(
        &self,
        rng: &mut R,
        _r: &Ray<T>,
        rec: &HitRecord<T, R>,
    ) -> Option<(Vector3<T>, Ray<T>)> {
        let target = rec.get_p() + rec.get_normal() + rand_in_unit_sphere(rng);
        let scattered = Ray::new(*rec.get_p(), target - rec.get_p());
        Some((self.albedo, scattered))
    }
}

pub struct Metal<T> {
    albedo: Vector3<T>,
    fuzz: T,
}

impl<T> Metal<T> {
    pub fn new(albedo: Vector3<T>, fuzz: T) -> Self {
        Self { albedo, fuzz }
    }
}

fn reflect<T>(v: Vector3<T>, n: Vector3<T>) -> Vector3<T>
where
    T: cgmath::BaseFloat,
{
    v - n * v.dot(n) * T::from(2.0).unwrap()
}

impl<T, R> Material<T, R> for Metal<T>
where
    T: cgmath::BaseFloat,
    Standard: Distribution<T>,
    R: rand::Rng,
{
    fn scatter(
        &self,
        rng: &mut R,
        r: &Ray<T>,
        rec: &HitRecord<T, R>,
    ) -> Option<(Vector3<T>, Ray<T>)> {
        let reflected = reflect(r.direction().normalize(), *rec.get_normal());
        let scattered = Ray::new(
            *rec.get_p(),
            reflected + rand_in_unit_sphere(rng) * self.fuzz,
        );
        if scattered.direction().dot(*rec.get_normal()) > T::zero() {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric<T> {
    ref_idx: T,
}

impl<T> Dielectric<T> {
    pub fn new(ref_idx: T) -> Self {
        Self { ref_idx }
    }
}

fn refract<T>(v: &Vector3<T>, n: &Vector3<T>, ni_over_nt: T) -> Option<Vector3<T>>
where
    T: cgmath::BaseFloat,
{
    let uv = v.normalize();
    let dt = uv.dot(*n);
    let discriminant = T::one() - ni_over_nt * ni_over_nt * (T::one() - dt * dt);
    if discriminant > T::zero() {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick<T: cgmath::BaseNum>(cosine: T, ref_idx: T) -> T {
    let r1 = (T::one() - ref_idx) / (T::one() + ref_idx);
    let r2 = r1 * r1;
    let r5 = r2 + (T::one() - r2) * (T::one() - cosine);
    r5 * r5 * r5 * r5 * r5
}

impl<T, R> Material<T, R> for Dielectric<T>
where
    T: cgmath::BaseFloat,
    Standard: Distribution<T>,
    R: rand::Rng,
{
    fn scatter(
        &self,
        rng: &mut R,
        r: &Ray<T>,
        rec: &HitRecord<T, R>,
    ) -> Option<(Vector3<T>, Ray<T>)> {
        let reflected = reflect(*r.direction(), *rec.get_normal());
        let normal = *rec.get_normal();

        let (outward_normal, ni_over_nt, cosine) = if r.direction().dot(normal) > T::zero() {
            (
                -normal,
                self.ref_idx,
                self.ref_idx * r.direction().dot(normal) / r.direction().magnitude(),
            )
        } else {
            (
                *rec.get_normal(),
                T::one() / self.ref_idx,
                -r.direction().dot(normal) / r.direction().magnitude(),
            )
        };

        let refracted = refract(r.direction(), &outward_normal, ni_over_nt);
        let reflect_prob = match refracted {
            None => T::one(),
            Some(_) => schlick(cosine, self.ref_idx),
        };

        let attenuation = vec3(T::one(), T::one(), T::one());
        if rng.gen::<T>() < reflect_prob {
            Some((attenuation, Ray::new(*rec.get_p(), reflected)))
        } else {
            Some((attenuation, Ray::new(*rec.get_p(), refracted.unwrap())))
        }
    }
}
