use std::io::{self, BufRead, Write};
use bcrypto::conv::hex_to_b64;
use bcrypto::oper::xor;

fn main() {
    test_xor_key();
}

fn user_input(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", prompt.to_string());
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line.pop();
    line
}

fn text_xor_key() -> {

}

fn test_xor() {
    let result1 = user_input("IN 1: ");
    let result2 = user_input("IN 2: ");
    println!("XOR: {:02X?}", xor(result1, result2));
}

fn test_hex_to_b64() {
    let result = user_input("HEX: ");
    println!("B64: {:?}", hex_to_b64(result));
}

