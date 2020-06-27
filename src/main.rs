mod perlin;

use bmp::{Image, Pixel};
use rand::{rngs::StdRng, SeedableRng};
#[macro_use]
extern crate bmp;

fn main() {
    let mut rng = StdRng::seed_from_u64(1337);
    let width = 1920;
    let height = 1080;
    let perlin_noise = perlin::Perlin::new(&mut rng);
    let mut img = Image::new(width, height);
    let scale = 30.0;
    for (x, y) in img.coordinates() {
        let val = perlin_noise.octave_noise(
            (x as f64 / width as f64) * scale,
            (y as f64 / height as f64) * scale,
            16,
            0.6,
        );
        let color = {
            if val <= 0.2 {
                px!(0, 0, (255.0 * (1.0 - val)) as u8)
            } else if val <= 0.4 {
                px!(0, 255.0 * val, (255.0 * (1.0 - 2.0 * val)) as u8)
            } else if val <= 0.6 {
                px!(0, (255.0 * (1.0 - 2.0 * val)) as u8, 0)
            } else if val <= 0.8 {
                px!(
                    (255.0 * (1.0 - 2.0 * val)) as u8,
                    (255.0 * 0.35 * val) as u8,
                    (255.0 * 0.125 * val) as u8
                )
            } else if val <= 0.9 {
                px!(
                    (255.0 * (1.0 - 4.0 * val)) as u8,
                    (255.0 * 0.35 * val) as u8,
                    (255.0 * 0.125 * val) as u8
                )
            } else {
                px!(
                    (255.0 * val) as u8,
                    (255.0 * val) as u8,
                    (255.0 * val) as u8
                )
            }
        };
        img.set_pixel(x, y, color);
    }
    img.save("test.bmp").unwrap();
}
