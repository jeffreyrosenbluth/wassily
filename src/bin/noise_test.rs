use noise::*;
use rand::prelude::*;
use rand_pcg::Pcg64;
use wassily::prelude::*;

fn main() {
    // let wk = WK::<[f64; 3], OpenSimplex>::new(8191.0, 8191.0, OpenSimplex::new());
    let wk = Noise::<_, 2>::new(8191.0, 8191.0, BasicMulti::default())
        .set_seed(1)
        .set_octaves(4)
        .set_frequency(3.0)
        .set_lacunarity(4.0)
        .set_persistence(1.0);
    let mut rng = Pcg64::seed_from_u64(0);
    let mut small = 0.0;
    let mut large = 0.0;
    let s = 0.0;
    let l = wk.width;
    for _ in 0..1_000_000 {
        let x = rng.gen_range(s..=l);
        let y = rng.gen_range(s..=l);
        // let z = rng.gen_range(s..=l);
        let n = wk.noise(x, y);
        if n < small {
            small = n
        };
        if n > large {
            large = n
        };
    }
    dbg!(small, large);
}
