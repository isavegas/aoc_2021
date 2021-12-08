use aoc_core::{AoCDay, ErrorWrapper, parse};

pub struct Day07;

type Num = u64;

fn gauss(n: Num) -> Num {
    (n * (n + 1)) / 2
}

impl AoCDay for Day07 {
    fn day(&self) -> usize {
        07
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("342730"), Some("92335207"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut crabs: Vec<Num> = parse(input, ",")?;
        crabs.sort();
        let alignment = *crabs.get(crabs.len()/2).unwrap();
        Ok(crabs.iter().map(|s| s.abs_diff(alignment)).sum::<Num>().to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut crabs: Vec<Num> = parse(input, ",")?;
        crabs.sort();
        // We need to do the division as floats to ensure we can round
        let alignment_a = crabs.iter().sum::<Num>() / crabs.len() as Num;
        let alignment_b = alignment_a + 1;
        let alignment_a_val = crabs.iter()
            .map(|s| gauss(s.abs_diff(alignment_a)))
            .sum::<Num>();
        let alignment_b_val = crabs.iter()
            .map(|s| gauss(s.abs_diff(alignment_b)))
            .sum::<Num>();
        Ok(std::cmp::min(alignment_a_val, alignment_b_val).to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day07)
}
