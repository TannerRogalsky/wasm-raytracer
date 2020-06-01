extern crate cgmath;

use cgmath::Vector3;

pub struct Ray<T> {
    a: Vector3<T>,
    b: Vector3<T>,
}

impl<T: cgmath::BaseNum> Ray<T> {
    pub fn new(a: Vector3<T>, b: Vector3<T>) -> Self {
        Ray { a, b }
    }

    pub fn origin(&self) -> &Vector3<T> {
        &self.a
    }

    pub fn direction(&self) -> &Vector3<T> {
        &self.b
    }

    pub fn point_at_parameter(&self, t: T) -> Vector3<T> {
        self.a + self.b * t
    }
}
