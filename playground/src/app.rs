use cgmath::{vec3, ElementWise, InnerSpace, Vector3};
use rand::prelude::*;
use raytracer::*;
use std::rc::Rc;

pub struct App {
    pixels: Vec<Pixel>,
    world: HitTableList<f64, SmallRng>,
    camera: Camera<f64>,
    rng: SmallRng,
    width: usize,
    height: usize,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = SmallRng::seed_from_u64(0);
        let world = gen_world(&mut rng);

        let pixels: Vec<Pixel> = vec![Pixel::default(); width * height];

        let camera = {
            let origin = vec3(13.0, 2.0, 3.0);
            let look_at = vec3(0.0, 0.0, 0.0);
            Camera::new(
                origin,
                look_at,
                Vector3::unit_y(),
                20.0,
                width as f64 / height as f64,
                0.1,
                10.0,
            )
        };

        Self {
            pixels,
            world,
            camera,
            rng,
            width,
            height,
        }
    }

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    fn color(&mut self, r: &Ray<f64>, depth: usize) -> Vector3<f64> {
        if depth < 50 {
            match self.world.hit(r, 0.001..std::f64::MAX) {
                None => {
                    let unit_direction = r.direction().normalize();
                    let t = 0.5 * (unit_direction.y + 1.0);
                    (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
                }
                Some(hit) => {
                    if let Some((attenuation, ray)) =
                        hit.get_material().scatter(&mut self.rng, r, &hit)
                    {
                        attenuation.mul_element_wise(self.color(&ray, depth + 1))
                    } else {
                        vec3(0.0, 0.0, 0.0)
                    }
                }
            }
        } else {
            vec3(0.0, 0.0, 0.0)
        }
    }

    pub fn draw(&mut self) {
        const AA_STEPS: usize = 1;
        let width = self.width;
        let height = self.height;

        let mut i = 0usize;
        for y in (0..height).rev() {
            for x in 0..width {
                let col = (0..AA_STEPS).fold(vec3(0.0, 0.0, 0.0), |acc, _i| {
                    let u = (x as f64 + self.rng.gen::<f64>()) / (width as f64);
                    let v = (y as f64 + self.rng.gen::<f64>()) / (height as f64);

                    let r = self.camera.ray(&mut self.rng, u, v);
                    acc + self.color(&r, 0)
                }) / AA_STEPS as f64;

                self.pixels[i].r = (col.x.sqrt() * 255.99) as u8;
                self.pixels[i].g = (col.y.sqrt() * 255.99) as u8;
                self.pixels[i].b = (col.z.sqrt() * 255.99) as u8;

                i += 1;
            }
        }
    }
}

fn gen_world<R>(rng: &mut R) -> HitTableList<f64, R>
where
    R: rand::Rng + 'static,
{
    let mut list = HitTableList::new();
    list.add(Box::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(vec3(0.5, 0.5, 0.5))),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = vec3(
                (a as f64) + 0.9 * rng.gen::<f64>(),
                0.2,
                (b as f64) + 0.9 * rng.gen::<f64>(),
            );
            if choose_mat < 0.8 {
                list.add(Box::new(Sphere::new(
                    center,
                    0.2,
                    Rc::new(Lambertian::new(vec3(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    ))),
                )));
            } else if choose_mat < 0.95 {
                list.add(Box::new(Sphere::new(
                    center,
                    0.2,
                    Rc::new(Metal::new(
                        vec3(
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                        ),
                        0.5 * rng.gen::<f64>(),
                    )),
                )));
            } else {
                list.add(Box::new(Sphere::new(
                    center,
                    0.2,
                    Rc::new(Dielectric::new(1.5)),
                )));
            }
        }
    }
    list.add(Box::new(Sphere::new(
        vec3(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    list.add(Box::new(Sphere::new(
        vec3(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(vec3(0.4, 0.2, 0.1))),
    )));
    list.add(Box::new(Sphere::new(
        vec3(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0)),
    )));
    list
}
