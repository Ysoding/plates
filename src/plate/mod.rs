pub mod constants;
pub mod prng;
use std::collections::HashSet;

use crate::plate::constants::{ALPHABET, ALPHANUMERIC, EMOJIS, PROVINCES};
use base32::{decode, encode, Alphabet};

pub struct PlateCode {}

impl PlateCode {
    pub fn encode(input: &str, emoji: bool) -> String {
        let encoded = encode(Alphabet::Rfc4648 { padding: false }, input.as_bytes());
        let seed = prng::get_seed_from_string(input);
        let mut random = prng::split_mix32(seed);

        let plates = encoded
            .as_bytes()
            .chunks(5)
            .map(|chunk| {
                let chunk_str = std::str::from_utf8(chunk).unwrap_or("AAAAA");
                let padded_chunk = format!("{:A<5}", chunk_str);

                let province = PROVINCES
                    .chars()
                    .nth((random() * PROVINCES.chars().count() as f64) as usize)
                    .unwrap();

                let letter = &ALPHABET
                    .chars()
                    .nth((random() * ALPHABET.len() as f64) as usize)
                    .unwrap();

                format!("{}{}·{}", province, letter, padded_chunk)
            })
            .collect::<Vec<_>>();

        if emoji {
            let emojis = Self::generate_unique_emojis(&mut random, plates.len() + 1);
            let start_emjoi = &emojis[0];
            let separator_emojis = &emojis[1..];

            let tmp = plates
                .into_iter()
                .zip(separator_emojis)
                .map(|(plate, emjoi)| format!("{} {}", plate, emjoi))
                .collect::<Vec<_>>()
                .join(" ")
                .to_string();

            format!("{} {}", start_emjoi, tmp)
        } else {
            plates.join(" ")
        }
    }

    pub fn decode(license_plates: &str) -> String {
        let encoded = license_plates
            .split(' ')
            .filter_map(|plate| plate.split('·').nth(1))
            .map(|part| part.trim_end_matches('A'))
            .collect::<Vec<_>>()
            .concat();

        let decoded_bytes =
            decode(Alphabet::Rfc4648 { padding: false }, &encoded).expect("Invalid base32 input");

        String::from_utf8(decoded_bytes).expect("Invalid UTF-8")
    }

    pub fn hash(value: &str, emoji: bool) -> String {
        let seed = prng::get_seed_from_string(value);
        let mut random = prng::split_mix32(seed);

        let province = PROVINCES
            .chars()
            .nth((random() * PROVINCES.chars().count() as f64) as usize)
            .unwrap();

        let letter = &ALPHABET
            .chars()
            .nth((random() * ALPHABET.len() as f64) as usize)
            .unwrap();

        let alphanumerics = Self::generate_random_alphanumeric(&mut random, 5);

        let base_plate = format!("{}{}·{}", province, letter, alphanumerics);

        if !emoji {
            base_plate
        } else {
            let start_emoji = EMOJIS[(random() * EMOJIS.len() as f64) as usize];
            let end_emoji = EMOJIS[(random() * EMOJIS.len() as f64) as usize];

            format!("{} {} {}", start_emoji, base_plate, end_emoji)
        }
    }

    fn generate_random_alphanumeric(random: &mut impl FnMut() -> f64, length: usize) -> String {
        (0..length)
            .map(|_| {
                let index = (random() * ALPHANUMERIC.len() as f64) as usize;
                ALPHANUMERIC.chars().nth(index).unwrap()
            })
            .collect()
    }

    fn generate_unique_emojis(random: &mut impl FnMut() -> f64, count: usize) -> Vec<String> {
        let mut unique_emojis = HashSet::new();
        while unique_emojis.len() != count {
            let emjoi = EMOJIS[(random() * EMOJIS.len() as f64) as usize].to_string();
            unique_emojis.insert(emjoi);
        }

        unique_emojis.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64() {
        // FIXME: 当使用A做为base32编码padding时，如果原有的尾部带有A，则解码会出错 。
        // AAAAA
        // ORSXG5A
        // ORSXG 5AAAA
        // tes
        println!("{:A<5}", "AAAA");
        let input = "test";
        let encoded = encode(Alphabet::Rfc4648 { padding: false }, input.as_bytes());
        println!("{}", encoded);

        let plates = encoded
            .as_bytes()
            .chunks(5)
            .map(|chunk| {
                let chunk_str = std::str::from_utf8(chunk).unwrap_or("AAAAA");
                let padded_chunk = format!("{:A<5}", chunk_str);

                padded_chunk
            })
            .collect::<Vec<_>>();
        let tmp_encoded = plates.join(" ");
        println!("{}", tmp_encoded);

        let encoded = tmp_encoded
            .split(' ')
            .map(|part| part.trim_end_matches('A'))
            .collect::<Vec<_>>()
            .concat();

        let decoded_bytes =
            decode(Alphabet::Rfc4648 { padding: false }, &encoded).expect("Invalid base32 input");

        println!(
            "{}",
            String::from_utf8(decoded_bytes).expect("Invalid UTF-8")
        );
    }

    #[test]
    fn test_hash() {
        println!("{}", PlateCode::hash("hello, test", true));
        println!("{}", PlateCode::hash("hello, test", false));
    }

    #[test]
    fn test_random() {
        let value = "aaddda";
        let seed = prng::get_seed_from_string(value);
        let mut random = prng::split_mix32(seed);

        let mut idx = (random() * PROVINCES.chars().count() as f64) as usize;
        let province = PROVINCES.chars().nth(idx).unwrap();
        println!("{}", province);

        idx = (random() * ALPHABET.len() as f64) as usize;
        let letter = &ALPHABET.chars().nth(idx).unwrap();
        println!("{}", letter);

        println!(
            "{}",
            PlateCode::generate_random_alphanumeric(&mut random, 5)
        );
    }
}
