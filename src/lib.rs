mod rand;

use std::{time::{UNIX_EPOCH,SystemTime}};
pub fn generate_random_sequence(n: usize) -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789";
    let mut rng = rand::PCG32::new();
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % 100_000_000_u128;
    //thread::sleep(Duration::from_millis(8));
    let mut cnt=0_u32;
    for _ in 0..23_000_000 {
        cnt += 1
    }
    let mut divisor = 240_043_u128;
    if cnt > 1005 {
        divisor -= 5
    }
    let seq = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % divisor;
    rng.seed(seed as u64, seq as u64); // use nano timer or another random source
    let high = charset.len() as f64;
    (0..n)
        .map(|_| {
            let idx = rng.gen_range(0.0,high) as usize;
            charset[idx] as char
        })
        .collect()
}