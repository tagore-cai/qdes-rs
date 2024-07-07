use std::collections::HashMap;

use base64::{prelude::BASE64_STANDARD, Engine};

fn head(b: &[u8]) -> Vec<u8> {
    let p = vec![21, 4, 9, 26, 16, 20, 27, 30];
    p.iter().map(|&x| b[x]).collect()
}

fn tail(b: &[u8]) -> Vec<u8> {
    let p = vec![18, 11, 3, 2, 1, 7, 6, 25];
    p.iter().map(|&x| b[x]).collect()
}

fn middle(b: &[u8]) -> Vec<u8> {
    let zd: HashMap<char, u8> = [
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('A', 10),
        ('B', 11),
        ('C', 12),
        ('D', 13),
        ('E', 14),
        ('F', 15),
    ]
    .iter()
    .cloned()
    .collect();
    let ol = vec![
        212, 45, 80, 68, 195, 163, 163, 203, 157, 220, 254, 91, 204, 79, 104, 6,
    ];
    let mut res: Vec<u8> = Vec::new();
    let mut j = 0;
    for i in (0..b.len()).step_by(2) {
        let one = zd[&(b[i] as char)];
        let two = zd[&(b[i + 1] as char)];
        let r = one * 16 ^ two;
        res.push(r ^ ol[j]);
        j += 1;
    }
    res
}

pub fn sign(params: &str) -> String {
    let md5_hash = md5::compute(params);
    let binding = format!("{:x}", md5_hash).to_uppercase();
    let md5_str = binding.as_bytes();
    println!("{:?}", md5_str);
    let h = head(&md5_str);
    let e = tail(&md5_str);
    let ls = middle(&md5_str);
    let m = BASE64_STANDARD.encode(ls);
    let mut res =
        String::from("zzb") + &String::from_utf8(h).unwrap() + &m + &String::from_utf8(e).unwrap();
    res.make_ascii_lowercase();
    res.retain(|c| c != '/' && c != '+' && c != '=');
    res
}