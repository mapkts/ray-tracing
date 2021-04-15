#[macro_use]
extern crate ray_tracing;

use ray_tracing::prelude::*;
use ray_tracing::hittable::*;
use ray_tracing::util::*;
use ray_tracing::sphere::Sphere;
use std::io;
use std::io::prelude::*;

fn ray_color(ray: &Ray, world: &impl Hittable, depth: u32) -> Rgb {
    // 0.001 here is for fixing shadow acne.
    if let Some(rec) = world.hit(ray, 0.001, INIFINTY) {
        // Gathers no more light if we exceeded the ray bounce limit.
        if depth <= 0 {
            return rgb!(0, 0, 0);
        }

        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    } else {
        let unit_direction = ray.direction.normal();

        // A trick that converts range from [-1, 1) to [0, 1)
        let t = 0.5 * (unit_direction.y + 1.0);

        return (1.0 - t) * rgb!(1.0, 1.0, 1.0) + t * rgb!(0.5, 0.7, 1.0);
    }
}

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    macro_rules! stdout {
        ($fmt:expr, $($arg:tt)*) => {
            write!(stdout, $fmt, $($arg)*).unwrap()
        };

        ($arg:tt) => {
            write!(stdout, "{}", $arg).unwrap()
        };
    }

    macro_rules! stderr {
        ($fmt:expr, $($arg:tt)*) => {
            write!(stderr, $fmt, $($arg)*).unwrap()
        };

        ($arg:tt) => {
            write!(stderr, "{}", $arg).unwrap()
        };
    }

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 700;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let camera = Camera::default();    

    // Render
    stdout!("P3\n{} {}\n255\n", image_width, image_height);

    for h in (0..image_height).rev() {
        // Prints how many scanlines left.
        stderr!("\rScanlines remaining: {}  ", h);

        for w in 0..image_width {
            let mut pixel_color = Rgb::default();
            for _s in 0..samples_per_pixel {
                let u = (w as f64 + random_f64()) / (image_width - 1) as f64;
                let v = (h as f64 + random_f64()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            pixel_color.write(&mut stdout, samples_per_pixel).unwrap();
        }
    }
}
