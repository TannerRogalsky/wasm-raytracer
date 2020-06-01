extern crate cgmath;

use super::Material;
use cgmath::Vector3;
use std::rc::Rc;

pub struct HitRecord<T, R> {
    t: T,
    p: Vector3<T>,
    normal: Vector3<T>,
    material: Rc<dyn super::Material<T, R>>,
}

impl<T: std::marker::Copy, R> HitRecord<T, R> {
    pub fn new(
        t: T,
        p: Vector3<T>,
        normal: Vector3<T>,
        material: Rc<dyn super::Material<T, R>>,
    ) -> Self {
        Self {
            t,
            p,
            normal,
            material,
        }
    }

    pub fn get_t(&self) -> T {
        self.t
    }

    pub fn set_t(&mut self, t: T) {
        self.t = t;
    }

    pub fn get_p(&self) -> &Vector3<T> {
        &self.p
    }

    pub fn set_p(&mut self, p: Vector3<T>) {
        self.p = p;
    }

    pub fn get_normal(&self) -> &Vector3<T> {
        &self.normal
    }

    pub fn set_normal(&mut self, normal: Vector3<T>) {
        self.normal = normal;
    }

    pub fn get_material(&self) -> &Rc<dyn Material<T, R>> {
        &self.material
    }
}

pub trait HitTable<T, R> {
    fn hit(&self, r: &super::ray::Ray<T>, t: std::ops::Range<T>) -> Option<HitRecord<T, R>>;
}

pub struct HitTableList<T, R> {
    list: std::vec::Vec<Box<dyn HitTable<T, R>>>,
}

impl<T, R> HitTableList<T, R> {
    pub fn new() -> Self {
        HitTableList { list: vec![] }
    }

    pub fn add(&mut self, ht: Box<dyn HitTable<T, R>>) {
        self.list.push(ht)
    }
}

impl<T: cgmath::BaseNum, R> HitTable<T, R> for HitTableList<T, R> {
    fn hit(&self, r: &super::ray::Ray<T>, t: std::ops::Range<T>) -> Option<HitRecord<T, R>> {
        let mut closest_so_far = t.end;
        let mut hit = None;
        for ht in self.list.iter() {
            if let Some(hc) = ht.hit(r, t.start..closest_so_far) {
                closest_so_far = hc.t;
                hit = Some(hc);
            }
        }
        hit
    }
}
