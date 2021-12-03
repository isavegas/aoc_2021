use aoc_core::{AoCDay, ErrorWrapper, parse_with};

// Expose struct and data_width field for tests,
// as the test data provided uses 5 bit data
pub struct Day03 {
    pub data_width: usize,
}

// Input data width is 12 bits, but a little extra won't hurt
type Num = u16;

fn parse_binary(input: &str) -> Result<Num, ErrorWrapper> {
    let mut val = 0;
    for (i, c) in input.chars().rev().enumerate() {
        match c {
            '1' => val |= 0x01 << i,
            '0' => (),
            _ => return Err(ErrorWrapper::ParsingError("Expected '1' or '0'".to_string())),
        }
    }
    Ok(val)
}

impl AoCDay for Day03 {
    fn day(&self) -> usize {
        03
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("4118544"), None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let data = parse_with::<Num>(input, parse_binary);
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
        let input_data = parse_with::<Num>(input, parse_binary);

        // Not very efficient, but that's fine considering the scale
        let find_rating = |cmp: fn(usize, usize) -> bool| -> u64 {
            let mut pos = self.data_width;
            let mut data = input_data.clone();
            while data.len() > 1 {
                pos -= 1;
                let count = data.iter()
                    .map(|d| (*d >> pos) & 0x01)
                    .filter(|d| *d > 0)
                    .count();
                let mut new_data = data.iter()
                    .filter(|d| (((**d >> pos) & 0x01) > 0) == cmp(count, data.len() - count))
                    .map(|d| *d)
                    .collect();
                std::mem::swap(&mut new_data, &mut data);
            }
            data[0] as u64
        };

        let o = find_rating(|a, b| a >= b);
        let c = find_rating(|a, b| a < b);
        Ok((o * c).to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day03 {
        data_width: 12,
    })
}
