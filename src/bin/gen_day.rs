macro_rules! impl_template {
    () => {
        r#"use aoc_core::{{AoCDay, ErrorWrapper}};

pub struct Day{0};

type Num = u64;

impl AoCDay for Day{0} {{
    fn day(&self) -> usize {{
        {0}
    }}
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {{
        (None, None)
    }}
    fn part1(&self, _input: &str) -> Result<String, ErrorWrapper> {{
        Err(ErrorWrapper::NotImplemented)
    }}
    fn part2(&self, _input: &str) -> Result<String, ErrorWrapper> {{
        Err(ErrorWrapper::NotImplemented)
    }}
}}

pub fn get_day() -> Box<dyn AoCDay> {{
    Box::new(Day{0})
}}
"#
    };
}

macro_rules! test_template {
    () => {
        r##"use aoc_2020::day::day_{0}::get_day;

const INPUT: &str = r#""#

#[test]
pub fn part1_1() {{
    assert_eq!(
        get_day()
            .part1(INPUT)
            .expect("Error"),
        "".to_string()
    );
}}

#[test]
pub fn part2_1() {{
    assert_eq!(
        get_day()
            .part2(INPUT)
            .expect("Error"),
        "".to_string()
    );
}}
"##
    }
}

use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

// TODO: Clean up and put in aoc_core?
pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if let Ok(num) = args[1].parse::<usize>() {
            let num_str = format!("{:02}", num);
            let day_path = format!(
                "{}/day_{}.rs",
                concat!(env!("CARGO_MANIFEST_DIR"), "/src/day"),
                num_str
            );
            let input_path = format!(
                "{}/day_{}.txt",
                concat!(env!("CARGO_MANIFEST_DIR"), "/src/input"),
                num_str
            );
            let mut f = BufWriter::new(
                OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(day_path.clone())
                    .expect("Unable to create file"),
            );
            write!(f, impl_template!(), num_str).expect("Unable to write file");
            println!("Output file created at {}", day_path);
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(input_path.clone())
                .expect("Unable to create input file");
            println!("Input file created at {}", input_path);
            let test_path = format!(
                "{}/day_{}.rs",
                concat!(env!("CARGO_MANIFEST_DIR"), "/tests"),
                num_str
            );
            let mut f = BufWriter::new(
                OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(test_path.clone())
                    .expect("Unable to create file"),
            );
            write!(f, test_template!(), num_str).expect("Unable to write file");
            println!("Test file created at {}", test_path);
        }
    }
}
