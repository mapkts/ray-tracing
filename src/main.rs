use std::io;
use std::io::prelude::*;
fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_height, image_width);

    let stdout = io::stdout();
    let mut lock = stdout.lock();
    for h in (0..image_height).rev() {
        write!(io::stderr(), "\rScanlines remaining: {}  ", h).expect("error writing stderr");
        for w in 0..image_width {
            let r = w as f64 / (image_width - 1) as f64;
            let g = h as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            write!(lock, "{} {} {}", ir, ig, ib).expect("error writing stdout");
        }
    }
    drop(lock);
}
