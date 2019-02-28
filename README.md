## ni-rs

Port of [ni.c](https://github.com/aoh/ni) to have a small data mutator in Rust.

## Usage

# Generate one mutation from a single buffer

```
use std::fs::File;
use std::io::Write;

fn main() {
    let input = include_bytes!("input");
    let mutation = ni_rs::mutate(input);
    let mut f = File::create("output").ok().unwrap();
    f.write(&mutation);
}
```

# Generate 32 mutations from a single buffer

```
fn main() {
    let input = include_bytes!("input");
    let mutations = ni_rs::mutate_n(input, 32);
    assert_eq!(mutations.len(), 32);
}
```

## Corpus mutation

# Generate one mutation from a corpus

```
use std::fs::File;
use std::io::Write;

fn main() {
    let samples = vec![
                "<test><SUPERINNER!/></test><h>",
                "<test></test>",
                "<bingo><yay></bingo>"
                ];
    let mutation = ni_rs::mutate_corpus(samples);
    let mut f = File::create("output").ok().unwrap();
    f.write(&mutation);
}
```

# Generate 24 mutations from a corpus

```
fn main() {
    let samples = vec![
                "<test><SUPERINNER!/></test><h>",
                "<test></test>",
                "<bingo><yay></bingo>"
                ];
    let mutations = ni_rs::mutate_samples(samples, 24);
    assert_eq!(mutations.len(), 32);
}
```

## Docs

`cargo doc` for more information
