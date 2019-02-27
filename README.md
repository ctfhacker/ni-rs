## ni-rs

Port of [ni.c](https://github.com/aoh/ni) to have a small data mutator in Rust.

## Usage

```
use std::fs::File;
use std::io::Write;

fn main() {
    let input = include_bytes!("input");
    let muts = ni_rs::mutate(input);
    let mut f = File::create("output").ok().unwrap();
    f.write(&muts);
}
```
