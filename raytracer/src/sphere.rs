extern crate cgmath;

use super::{HitRecord, HitTable, Material, Ray};
use cgmath::InnerSpace;
use std::ops::Range;
use std::sync::Arc as Rc;

pub struct Sphere<T, R> {
    center: cgmath::Vector3<T>,
    radius: T,
    // this could, theoretically, be a reference but doing the lifetimes sounds unfun
    material: Rc<dyn Material<T, R> + Send + Sync>,
}

impl<T, R> Sphere<T, R> {
    pub fn new(
        center: cgmath::Vector3<T>,
        radius: T,
        material: Rc<dyn Material<T, R> + Send + Sync>,
    ) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<T: cgmath::BaseNum, R> Sphere<T, R> {
    pub fn hit_record(&self, ray: &Ray<T>, t: T) -> HitRecord<T, R> {
        let p = ray.point_at_parameter(t);
        let normal = (p - self.center) / self.radius;
        HitRecord::new(t, p, normal, Rc::clone(&self.material))
    }
}

impl<T: cgmath::BaseFloat, R> HitTable<T, R> for Sphere<T, R> {
    fn hit(&self, r: &Ray<T>, t: Range<T>) -> Option<HitRecord<T, R>> {
        let oc = r.origin() - self.center;
        let a = r.direction().magnitude2();
        let b = oc.dot(*r.direction());
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = b * b - a * c;
        // todo: making 0 a constant would be an improvement https://github.com/rust-num/num-traits/issues/54
        if discriminant > T::zero() {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t.end && temp > t.start {
                return Some(self.hit_record(r, temp));
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t.end && temp > t.start {
                return Some(self.hit_record(r, temp));
            }
        }
        None
    }
}
