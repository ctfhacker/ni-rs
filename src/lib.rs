#![feature(asm)]
#![feature(exclusive_range_pattern)]

use std::io::Write;
use std::str;

const AIMROUNDS: usize = 256;
const AIMAX: usize = 512;
const AIMLEN: usize = 1024;
const BUFSIZE: usize = 4096;

fn rdrand() -> usize {
    let ret: u64;
    unsafe { asm!("rdrand $0" : "=r"(ret)) }
    ret as usize
}

/// Generate a random character
fn rand_char() -> char {
    ((rdrand() % 255) as u8) as char
}

fn random(end: usize) -> usize {
    if end == 0 {
        return 0;
    }

    rdrand() % end
}

/// Generate two random numbers that are not the same
/// returns two numbers, the first is less than the second
fn two_rand_numbers(len: usize) -> (usize, usize) {
    let mut a = rdrand() % len;
    let mut b = rdrand() % len;
    loop {
        if a != b {
            break;
        }
        a = rdrand() % len;
        b = rdrand() % len;
    }

    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

fn sufscore(a: &str, b: &str) -> usize {
    let mut last = 255 as char;
    let mut n = 0;
    let mut alen = a.len();
    let mut blen = b.len();
    for (a_char, b_char) in a.chars().zip(b.chars()) {
        if n < AIMAX {
            break;
        }

        if a_char == b_char {
            break;
        }

        if a_char != last {
            last = a_char;
            n += 32;
        }
    }

    n
}

fn aim(from: &str, to: &str, jump: &mut usize, land: &mut usize) {
    println!("In aim");
    let fend = from.len();
    let tend = to.len();
    let mut best_score = 0;
    let mut score = 0;
    let rounds = 0;
    if fend == 0 {
        *jump = 0;
        *land = random(tend);
        return;
    }

    if tend == 0 {
        *land = 0;
        return;
    }

    *jump = random(fend);
    *land = random(tend);

    let rounds = random(AIMROUNDS);
    let mut j = random(fend);
    let mut l = random(tend);
    for _ in 0..rounds {
        let mut maxs = AIMLEN;
        j = random(fend);
        l = random(tend);
        while maxs > 0 && l < tend && from.chars().take(j).next() != to.chars().take(l).next() {
            l += 1;
            maxs -= 1;
        }

        score = sufscore(&from[j..], &to[l..]);
        if score > best_score {
            best_score = score;
            *jump = j;
            *land = l;
        }
    }
}

/// Generate a random substring from one of the given corpus samples
fn random_block<'a>(data: &'a str, samples: Vec<&'a str>) -> &'a str {
    let rand_index = random(samples.len());
    let rand_sample = samples[rand_index];
    if rand_sample.len() < 3 {
        return data;
    }

    let start = random(rand_sample.len() - 2);
    let mut len = rand_sample.len() - start;
    if len > 4 * data.len() {
        len = 4 * data.len();
    }
    len = random(len);
    &rand_sample[start..]
}

/// Returns the start and end indeces of a random number in the buffer
fn seek_num(data: &str) -> Option<(usize, usize)> {
    let end = data.len();
    let rand_start = random(end);
    let mut start_index = None;
    let mut end_index = None;
    for (i, c) in data[rand_start..].char_indices() {
        match (c, start_index) {
            // First time seeing a number
            ('0'..'9', None) => start_index = Some(i + rand_start),

            // Still seeing a number, continue
            ('0'..'9', Some(_)) => continue,

            // Saw some number, and are no longer seeing digits
            (_, Some(_)) => {
                end_index = Some(i + rand_start);
                break;
            }

            // Everything else
            _ => continue,
        }
    }

    if let (Some(start), Some(end)) = (start_index, end_index) {
        // If found both start and end, deconstruct to a single Option
        Some((start, end))
    } else {
        // No number found
        None
    }
}

/// Randomly changes or bit flips a number
fn twiddle(val: usize) -> i64 {
    let mut result = val as i64;
    loop {
        match random(3) {
            // Make a new random i64 number
            0 => result = random(i64::max_value() as usize) as i64,
            // Flip one of the result bits
            1 => result ^= 1 << random(64 - 1),
            // Add a number relatively close to 0
            2 => result += random(5) as i64 - 2,
            _ => continue,
        }

        // Continue twiddling 50% of the time
        if rdrand() & 1 == 0 {
            break;
        }
    }
    result
}

pub fn mutate_area<W: Write>(data: &str, samples: Vec<&str>, output: &mut W) {
    let mut end = data.len();
    loop {
        // let r = rdrand() % 24;
        let r = 26;
        println!("r: {}", r);
        match r {
            // match 7 {
            0 => {
                // Insert a random byte
                let position = random(end);
                write!(output, "{}", &data[..position]);
                write!(output, "{}", &data[..1]);
                write!(output, "{}", &data[position..]);
                return;
            }
            1 => {
                // Delete a random byte
                let position = random(end);
                if position + 1 >= end {
                    continue;
                }
                write!(output, "{}", &data[..position]);
                write!(output, "{}", &data[position + 1..]);
                return;
            }
            2..4 => {
                // Jump / Overlapping sequences
                if end == 0 {
                    continue;
                }

                // Generate two random numbers where a < b
                let (a, b) = two_rand_numbers(end);
                write!(output, "{}", &data[..a]);
                write!(output, "{}", &data[b..]);
                return;
            }
            4..6 => {
                // Repeat characters
                if end == 0 {
                    continue;
                }

                let mut n = 8;
                while rdrand() & 1 == 0 && n < 20000 {
                    n <<= 1;
                }

                n = rdrand() % n + 2;

                // Generate two random numbers where a < b
                let (s, e) = two_rand_numbers(end);
                let mut len = e - s;

                write!(output, "{}", &data[..s]);

                if len * n > 0x8000000 {
                    len = rdrand() % 1024 + 2;
                }

                // Insert some substring `n` times
                for _ in 0..n {
                    write!(output, "{}", &data[s..s + len]);
                }

                // Write the rest of the string
                write!(output, "{}", &data[s..]);
                return;
            }
            6 => {
                // Insert random data
                let position = random(end);
                let n = rdrand() % 1022 + 2;
                write!(output, "{}", &data[..position]);
                for i in 0..n {
                    write!(output, "{}", rand_char());
                }
                write!(output, "{}", &data[position..]);
                return;
            }
            7..13 => {
                // Aimed jump to self
                if end < 5 {
                    continue;
                }

                let mut j = 0;
                let mut l = 0;
                aim(data, data, &mut j, &mut l);

                println!("data: {} j: {}, l: {}", data, j, l);
                write!(output, "{}", &data[..j]);
                write!(output, "{}", &data[l..]);
                return;
            }
            13..22 => {
                if end < 8 {
                    continue;
                }

                let random_chunk = random_block(data, samples);
                let stend = random_chunk.len();
                let dm = end >> 1;
                let sm = stend >> 1;
                let mut j = 0;
                let mut l = 1;
                aim(data, random_chunk, &mut j, &mut l);
                write!(output, "{}", &data[..j]);

                end -= j;
                let buff = &random_chunk[l..];
                aim(buff, &data[j..], &mut j, &mut l);
                write!(output, "{}", &buff[..j]);
                write!(output, "{}", &data[l..]);
                return;
            }
            22..24 => {
                // Insert semirandom bytes
                if end == 0 {
                    continue;
                }

                let mut n = random(BUFSIZE);
                let position = random(end);
                for _ in 0..5 {
                    n = random(n);
                }

                if n == 0 {
                    n = 2;
                }
                write!(output, "{}", &data[..position]);
                for _ in 0..n {
                    let r = random(end);
                    write!(output, "{}", data.chars().take(r - 1).next().unwrap());
                }
                write!(output, "{}", &data[position..]);
                return;
            }
            24 => {
                // Overwrite semirandom bytes
                if end < 2 {
                    continue;
                }

                let a = random(end - 2);
                let b = match rdrand() & 1 {
                    0 => random(std::cmp::min(BUFSIZE - 2, end - a - 2)) + a + 2,
                    _ => random(32) + a + 2,
                };

                write!(output, "{}", &data[..a]);
                for _ in a..b {
                    let r = random(end - 1);

                    // Access to a single character in &str
                    write!(output, "{}", &data[r..r + 1]);
                }

                // Possible b can be longer than data
                if end > b {
                    write!(output, "{}", &data[b..]);
                }
                return;
            }
            25..29 => {
                if end == 0 {
                    continue;
                }

                let mut result = None;

                // Attempt to find a number at a random location in the data buffer
                for _ in 0..random(AIMROUNDS) {
                    if result.is_some() {
                        break;
                    }

                    result = seek_num(data);
                }

                match result {
                    Some((num_start, num_end)) => {
                        println!("{} -> {}", data, &data[num_start..num_end]);
                        // Write the data before the number
                        write!(output, "{}", &data[..num_start]);

                        // Try to parse the found number into a usize
                        if let Ok(num) = data[num_start..num_end].parse::<usize>() {
                            // Write the twiddled number
                            let twiddled = if num == 0 { twiddle(0) } else { twiddle(num) };
                            println!("{} -> {}", num, twiddled);
                            write!(output, "{}", twiddled);
                        }

                        // Write the rest of the buffer
                        write!(output, "{}", &data[num_end..]);
                    }
                    _ => {
                        // Did not find a number in the data buffer
                        // Continue to try a different mutation method
                        println!("Did not find number");
                        continue;
                    }
                }

                return;
            }
            29..35 => {
                // delimited swap

                return;
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut output = Vec::new();
        let sample = "1234567890";
        let samples = vec![sample];
        mutate_area(sample, samples, &mut output);
        assert_eq!(str::from_utf8(&output).unwrap(), "bbbb")
    }

    /*
    #[test]
    fn test2() {
        let mut output = Vec::new();
        mutate_area("bbbbbb", &mut output);
        assert_eq!(str::from_utf8(&output).unwrap(), "bbbb")
    }
    */
}
