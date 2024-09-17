// https://theartincode.stanis.me/008-djb2/
pub fn get_seed_from_string(value: &str) -> u32 {
    value
        .chars()
        .fold(5381, |acc, c| acc.wrapping_mul(33).wrapping_add(c as u32))
}

// https://rosettacode.org/wiki/Pseudo-random_numbers/Splitmix64#Go
// https://github.com/bryc/code/blob/master/jshash/PRNGs.md#splitmix32
pub fn split_mix32(seed: u32) -> impl FnMut() -> f64 {
    let mut seed = seed;
    move || {
        seed = seed.wrapping_add(0x9E3779B9);
        let mut hash = seed ^ (seed >> 16);
        hash = hash.wrapping_mul(0x21F0AAAD);
        hash ^= hash >> 15;
        hash = hash.wrapping_mul(0x735A2D97);
        (hash ^ (hash >> 15)) as f64 / 4294967296.0
    }
}
