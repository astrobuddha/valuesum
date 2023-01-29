use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MAP: HashMap<char, u32> = {
        let data = vec![
            ('a', 1),
            ('b', 2),
            ('c', 3),
            ('d', 4),
            ('e', 5),
            ('f', 6),
            ('g', 7),
            ('h', 8),
            ('i', 9),
            ('j', 10),
            ('k', 11),
            ('l', 12),
            ('m', 13),
            ('n', 14),
            ('o', 15),
            ('p', 16),
            ('q', 17),
            ('r', 18),
            ('s', 19),
            ('t', 20),
            ('u', 21),
            ('v', 22),
            ('w', 23),
            ('x', 24),
            ('y', 25),
            ('z', 26),
        ];

        data.into_iter().collect()
    };
}

pub struct Valuator;

impl Valuator {
    pub fn new() -> Valuator {
        Valuator
    }

    pub fn evaluate(&self, words: &str) -> u32 {
        let mut sum = 0;

        for c in words.to_lowercase().chars() {
            if let Some(k) = MAP.get(&c) {
                sum += k;
            }
        }

        sum
    }
}
