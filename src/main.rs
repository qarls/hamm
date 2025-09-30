use clap::Parser;
use std::cmp::min;
use std::{fs, path::PathBuf};

const ABOUT_MESSAGE: &str = "Compute the Hamming distance between s and t.";

#[derive(Parser)]
#[command(version, about = ABOUT_MESSAGE, long_about = None)]
struct Cli {
    /// Input file containing two lines of equal length DNA strands
    input_file: PathBuf,
}

// Function to check if singleline literal only contains CGAT
fn proof_dna(strand: &str) -> bool {
    for base in strand.as_bytes() {
        if let 67 | 71 | 65 | 84 = base {
        } else {
            return false;
        }
    }
    return true;
}

// Count unmatched bytes of two literals to the length of shortest literal.
fn count_unmatched_bytes(s: &str, t: &str) -> usize {
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    let mut count: usize = 0;

    for i in 0..min(s_bytes.len(), t_bytes.len()) {
        if s_bytes[i] != t_bytes[i] {
            count += 1;
        }
    }
    count
}

fn main() {
    let args = Cli::parse();
    let input = fs::read_to_string(&args.input_file).expect("Could not read this file.");
    let mut input_lines = input.lines();

    // Define s and t.
    let s: String;
    let t: String;
    match input_lines.clone().count() {
        ..=1 => panic!("Less than two lines detected."),
        2 => {
            //let vec = input_lines.collect::<Vec<_>>();
            //s = vec[0].to_string();
            //t = vec[1].to_string();
            s = input_lines.next().expect("s is empty!").to_string();
            t = input_lines.next().expect("t is empty!").to_string();
        }
        3.. => panic!("More than two lines detected."),
    };

    // Check if DNA strings are valid.
    {
        if s.len() != t.len() {
            panic!("Strings not of same length.")
        }
        if !proof_dna(&s) {
            panic!("s contains non-viable char!")
        }
        if !proof_dna(&t) {
            panic!("t contains non-viable char!")
        }
    }

    println!("{}", count_unmatched_bytes(&s, &t));
}
