use std::io;
use std::io::prelude::*;

use ray_tracing::ray::*;
use ray_tracing::sphere;
use ray_tracing::vector::*;

fn ray_color(ray: &Ray) -> Color {
    let t = sphere::hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, &ray);

    if t > 0.0 {
        let normal = (ray.at(t) - (0.0, 0.0, -1.0).into()).normalize();
        return 0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }

    let unit_direction = ray.direction.normalize();

    // Trick that converts y's range from [-1.0, 1.0] to [0, 1]
    let t = 0.5 * (unit_direction.y + 1.0);

    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
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
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_heigth = 2.0;
    let viewport_width = aspect_ratio * viewport_heigth;
    let focal_height = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_heigth, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_height);

    // Render
    stdout!("P3\n{} {}\n255\n", image_width, image_height);

    for h in (0..image_height).rev() {
        if h % 32 == 0 {
            stderr!("\rScanlines remaining: {}  ", h);
        }
        for w in 0..image_width {
            let r = w as f32 / (image_width - 1) as f32;
            let g = h as f32 / (image_height - 1) as f32;
            let ray = Ray::new(
                origin,
                lower_left_corner + r * horizontal + g * vertical - origin,
            );
            let color = ray_color(&ray);

            color.write(&mut stdout, 1).unwrap();
        }
    }
}
