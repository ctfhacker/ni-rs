extern crate ni_rs;
use std::str;

fn main() {
    let buffer = "<xml>test</xml>";
    let mut output = Vec::new();
    let samples = vec![buffer, "<xml>NUMTWO</xml>"];
    ni_rs::mutate_area(buffer, samples, &mut output);
    println!("{} -> {}", buffer, str::from_utf8(&output).unwrap());
}
