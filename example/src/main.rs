use std::fs::File;
use std::io::Write;

fn main() {
    let input = include_bytes!("input").to_vec();
    let muts = ni_rs::mutate(input);
    let mut f = File::create("output").ok().unwrap();
    f.write(&muts);
}
