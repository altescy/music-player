use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_charset() -> Vec<char> {
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect()
}

fn int_to_code(x: usize, charset: &Vec<char>) -> String {
    let mut x = x;
    let mut ret = String::new();
    while x != 0 {
        ret.push(charset[x % charset.len()]);
        x /= charset.len();
    }

    ret
}

pub fn gen_token<T: Hash>(t: &T) -> String {
    let token_len = 8;
    let charset = get_charset();

    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    let x: u64 = s.finish();

    int_to_code(x as usize % charset.len().pow(token_len) + 1, &charset)
}
