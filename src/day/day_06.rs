use aoc_core::{AoCDay, ErrorWrapper, parse};

pub struct Day06;

// Pure stack simulation
fn simulate(fish: &[usize], days: usize) -> usize {
    // Create buckets for our fish. new_buckets for
    // newly spawned fish that require an extra 2 days
    // before they can begin reproducing.
    let mut new_buckets: [usize; 2] = [0; 2];
    let mut buckets: [usize; 7] = [0; 7];
    for f in fish {
        let age = *f as usize;
        if age >= 7 {
            new_buckets[age - 7] += 1;
        } else {
            buckets[age] += 1;
        }
    }
    for _ in 0..days {
        let new_fish = buckets[0];

        buckets.rotate_left(1);
        buckets[6] += new_buckets[0];
        new_buckets.rotate_left(1);
        new_buckets[1] = new_fish;
    }
    buckets.iter().sum::<usize>() + new_buckets.iter().sum::<usize>()
}

impl AoCDay for Day06 {
    fn day(&self) -> usize {
        06
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("387413"), Some("1738377086345"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let fish: Vec<usize> = parse(input, ",")?;
        Ok(simulate(&fish, 80).to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let fish: Vec<usize> = parse(input, ",")?;
        Ok(simulate(&fish, 256).to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day06)
}
