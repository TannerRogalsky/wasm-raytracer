use super::Ray;
use cgmath::{vec3, InnerSpace, Vector3};
use rand::distributions::Standard;
use rand::prelude::*;

pub struct Camera<T> {
    origin: Vector3<T>,
    lower_left_corner: Vector3<T>,
    horizontal: Vector3<T>,
    vertical: Vector3<T>,
    u: Vector3<T>,
    v: Vector3<T>,
    #[allow(unused)]
    w: Vector3<T>,
    lens_radius: T,
}

impl Camera<f64> {
    pub fn new(
        origin: Vector3<f64>,
        look_at: Vector3<f64>,
        up: Vector3<f64>,
        v_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = v_fov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (origin - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        Self {
            origin,
            lower_left_corner: origin
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }
}

// FIXME: this shouldn't acquire it's own rng
fn rand_in_unit_disk<T, R>(rng: &mut R) -> Vector3<T>
where
    T: cgmath::BaseFloat,
    Standard: Distribution<T>,
    R: rand::Rng,
{
    let one = T::one();
    let two = one + one;
    loop {
        let x = rng.gen::<T>();
        let y = rng.gen::<T>();
        let p = vec3(x, y, T::zero()) * two - vec3(one, one, T::zero());
        if p.magnitude2() < one {
            return p;
        }
    }
}

impl<T> Camera<T>
where
    T: cgmath::BaseFloat,
    Standard: Distribution<T>,
{
    pub fn ray<R>(&self, rng: &mut R, u: T, v: T) -> Ray<T>
    where
        R: rand::Rng,
    {
        use cgmath::ElementWise;

        let rd = rand_in_unit_disk(rng).mul_element_wise(self.lens_radius);
        let offset = self.u.mul_element_wise(rd.x) + self.v.mul_element_wise(rd.y);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
