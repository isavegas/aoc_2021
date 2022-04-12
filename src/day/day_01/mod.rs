use aoc_core::{AoCDay, ErrorWrapper, parse_lines};

pub struct Day01;

type Num = u64;

fn window_sum(values: &Vec<Num>, last: usize) -> Num {
    values[last] + values[last-1] + values[last-2]
}

impl AoCDay for Day01 {
    fn day(&self) -> usize {
        01
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("1301"), Some("1346"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let measurements: Vec<Num> = parse_lines(input)?;
        let mut counter: usize = 0;
        for i in 1..measurements.len() {
            if measurements[i] > measurements[i-1] {
                counter += 1;
            }
        }
        Ok(counter.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let measurements: Vec<Num> = parse_lines(input)?;
        let mut counter: usize = 0;
        for i in 3..measurements.len() {
            if window_sum(&measurements, i) > window_sum(&measurements, i-1) {
                counter += 1;
            }
        }
        Ok(counter.to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day01)
}
