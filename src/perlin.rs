use rand::{rngs::StdRng, seq::SliceRandom};

pub struct Perlin {
    permutations: Vec<u8>,
}

fn fade(val: f64) -> f64 {
    let val_3 = val * val * val;
    let val_4 = val_3 * val;
    let val_5 = val_4 * val;
    6.0 * val_5 - 15.0 * val_4 + 10.0 * val_3
}

fn lerp(val1: f64, val2: f64, weight: f64) -> f64 {
    val1 + weight * (val2 - val1)
}

fn gradient(x: f64, y: f64, hash: u8) -> f64 {
    let hash = hash % 8;
    match hash {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        3 => -x - y,
        4 => std::f64::consts::FRAC_1_SQRT_2 * x + std::f64::consts::FRAC_1_SQRT_2 * y,
        5 => -std::f64::consts::FRAC_1_SQRT_2 * x + std::f64::consts::FRAC_1_SQRT_2 * y,
        6 => std::f64::consts::FRAC_1_SQRT_2 * x - std::f64::consts::FRAC_1_SQRT_2 * y,
        7 => -std::f64::consts::FRAC_1_SQRT_2 * x - std::f64::consts::FRAC_1_SQRT_2 * y,
        _ => panic!(),
    }
}

impl Perlin {
    pub fn new(rng: &mut StdRng) -> Self {
        let mut permutations: Vec<u8> = (0..=255).collect();
        permutations.shuffle(rng);
        Perlin { permutations }
    }

    fn hash(&self, x: u8, y: u8) -> u8 {
        let inner_idx = self.permutations[x as usize];
        let outer_idx = (inner_idx as usize + y as usize) % 256;
        self.permutations[outer_idx]
    }

    pub fn noise(&self, x: f64, y: f64) -> f64 {
        let x0 = (x.floor() as u32 % 256) as u8;
        let x1 = x0.wrapping_add(1);
        let y0 = (y.floor() as u32 % 256) as u8;
        let y1 = y0.wrapping_add(1);
        let boxed_x = x.fract();
        let weight_x = fade(boxed_x);
        let boxed_y = y.fract();
        let weight_y = fade(boxed_y);
        let top_left = self.hash(x0, y0);
        let top_right = self.hash(x1, y0);
        let bot_left = self.hash(x0, y1);
        let bot_right = self.hash(x1, y1);
        let interp1 = lerp(
            gradient(boxed_x, boxed_y, top_left),
            gradient(boxed_x - 1.0, boxed_y, top_right),
            weight_x,
        );
        let interp2 = lerp(
            gradient(boxed_x, boxed_y - 1.0, bot_left),
            gradient(boxed_x - 1.0, boxed_y - 1.0, bot_right),
            weight_x,
        );
        (lerp(interp1, interp2, weight_y) + 1.0) / 2.0
    }

    pub fn octave_noise(&self, x: f64, y: f64, octaves: u32, persistence: f64) -> f64 {
        let mut total = 0.0;
        let mut freq = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;
        for _ in 0..octaves {
            total += self.noise(x * freq, y * freq) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            freq *= 2.0;
        }
        total / max_value
    }
}
