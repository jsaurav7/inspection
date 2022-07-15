const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const BASE: i32 = CHARS.len() as i32;

pub fn encode(input: i32) -> String {
    if input == 0 {
        return CHARS.chars().next().unwrap().to_string();
    }
    let mut number = input;
    let mut result = String::new();
    loop {
        if number == 0 {
            break;
        };
        result.insert(0, CHARS.chars().nth((number % BASE) as usize).unwrap());
        number = number / BASE;
    }
    result
}

pub fn decode(input: &str) -> u64 {
    input.chars().rev().enumerate().fold(0, |sum, (i, chr)| {
        sum + (CHARS.find(chr).unwrap() as u64) * (BASE as u64).pow(i as u32)
    })
}
