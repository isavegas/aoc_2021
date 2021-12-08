use aoc_core::{AoCDay, ErrorWrapper, parse_lines_with};

// Expose struct and data_width field for tests,
// as the test data provided uses 5 bit data
pub struct Day03 {
    pub data_width: usize,
}

// Input data width is 12 bits, but a little extra won't hurt
type Num = u16;

impl AoCDay for Day03 {
    fn day(&self) -> usize {
        03
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("4118544"), None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let data: Vec<Num> = parse_lines_with(input, |s| Num::from_str_radix(s, 2).map_err(ErrorWrapper::from))?;
        let mut gamma_rate: u64 = 0;
        let mut epsilon_rate: u64 = 0;
        let mut counters: [usize; 16] = [0; 16];
        for value in data.iter() {
            for r in 0..self.data_width {
                if (value >> r & 0x01) > 0 {
                    counters[r] += 1;
                }
            }
        }
        for (r, count) in counters.iter().take(self.data_width).enumerate() {
            if *count >= data.len() / 2 {
                gamma_rate |= 0x01 << r;
            } else {
                epsilon_rate |= 0x01 << r;
            }
        }
        Ok((gamma_rate * epsilon_rate).to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let input_data: Vec<Num> = parse_lines_with(input, |s| Num::from_str_radix(s, 2).map_err(ErrorWrapper::from))?;

        // Not very efficient, but that's fine considering the scale
        let find_rating = |cmp: fn(usize, usize) -> bool| -> u64 {
            let mut pos = self.data_width;
            // Clone our data set, as we don't want to clobber the original
            let mut data = input_data.clone();
            while data.len() > 1 {
                pos -= 1;
                let count = data.iter()
                    .map(|d| (*d >> pos) & 0x01)
                    .filter(|d| *d > 0)
                    .count();
                let new_data = data.iter()
                    .filter(|d| (((**d >> pos) & 0x01) > 0) == cmp(count, data.len() - count))
                    .map(|d| *d)
                    .collect();
                data = new_data;
            }
            // Avoid overflowing due to our use of u16 for data points
            data[0] as u64
        };

        Ok((find_rating(|a, b| a >= b) * find_rating(|a, b| a < b)).to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day03 {
        data_width: 12,
    })
}
