use aoc_core::{AoCDay, ErrorWrapper, parse, bail, ensure};

pub struct Day16;

type Num = u64;

impl AoCDay for Day16 {
    fn day(&self) -> usize {
        16
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self, _input: &str) -> Result<String, ErrorWrapper> {
        Err(ErrorWrapper::NotImplemented)
    }
    fn part2(&self, _input: &str) -> Result<String, ErrorWrapper> {
        Err(ErrorWrapper::NotImplemented)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day16)
}
