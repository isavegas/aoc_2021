use aoc_core::{AoCDay, ErrorWrapper, parse_lines_with};
use std::str::FromStr;
use std::convert::TryFrom;
use bitflags::bitflags;

pub struct Day08;

bitflags! {
    struct Segments: u8 {
        const NONE = 0b00000000;
        const A    = 0b00000001;
        const B    = 0b00000010;
        const C    = 0b00000100;
        const D    = 0b00001000;
        const E    = 0b00010000;
        const F    = 0b00100000;
        const G    = 0b01000000;
    }
}

impl Segments {
    fn new() -> Self {
        Segments::NONE
    }
    fn count(&self) -> usize {
        let mut c: usize = 0;
        for i in 0..8 {
            c += ((self.bits >> i) & 0b00000001) as usize;
        }
        c
    }
}

impl TryFrom<char> for Segments {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            'e' => Ok(Self::E),
            'f' => Ok(Self::F),
            'g' => Ok(Self::G),
            _ => Err("Invalid character!"),
        }
    }
}

impl FromStr for Segments {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = Segments::new();
        for c in s.chars() {
            segments |= Segments::try_from(c)?;
        }
        Ok(segments)
    }
}

// Probably way more complicated than it needs to be, but it works.
fn deduce_digits(input: &Vec<Segments>, output: &Vec<Segments>) -> Vec<usize> {
    let mut map: [Segments; 10] = [Segments::NONE; 10];
    for segments in input {
        match segments.count() {
                2 => map[1] = *segments,
                4 => map[4] = *segments,
                3 => map[7] = *segments,
                7 => map[8] = *segments,
                _ => (),
        }
    }
    let top = map[7] ^ map[1];
    let top_bottom_left = map[4] ^ map[1];

    map[9] = *input.iter().filter(|s| !map.contains(s) && s.count() == 6).find(|segment|
        (**segment ^ (map[7] | map[4])).count() == 1
    ).expect("Cannot find 9!");

    let bottom = map[9] ^ (map[1] | top_bottom_left | top);

    map[3] = *input.iter().filter(|s| !map.contains(s) && s.count() == 5).find(|segment|
        (**segment ^ (map[9] ^ top_bottom_left)).count() == 1
    ).expect("Cannot find 3!");

    let top_left = map[3] ^ map[9];

    map[0] = *input.iter().find(|segment|
        !map.contains(segment) && segment.count() == 6 && ((**segment ^ (top | bottom | map[1] | top_left)).count() == 1)
    ).expect("Cannot find 0!");

    let left = map[0] ^ (map[1] | top | bottom);

    map[5] = *input.iter().filter(|s| !map.contains(s) && s.count() == 5).find(|segment|
        (**segment & left) == top_left
    ).expect("Cannot find 5!");

    map[6] = *input.iter().filter(|s| !map.contains(s) && s.count() == 6).find(|segment|
        (**segment & top_left) == top_left
    ).expect("Cannot find 6!");

    map[2] = *input.iter().filter(|s| !map.contains(s) && s.count() == 5)
        .next()
        .expect("Cannot find 2!");

    output.iter().map(|o| map.iter().position(|v| o == v).expect("Missing digit!")).collect()
}

fn parse(line: &str) -> Result<(Vec<Segments>, Vec<Segments>), ErrorWrapper> {
    let mut parts = line.split('|').map(str::trim);
    let input = parts.next().ok_or_else(||ErrorWrapper::ParseError("Missing input".to_string()))?;
    let output = parts.next().ok_or_else(||ErrorWrapper::ParseError("Missing output".to_string()))?;
    assert_eq!(parts.next(), None, "Too many parts!");
    Ok((
        input.split(' ').map(|s| Segments::from_str(s).expect("Invalid digit in input")).collect(),
        output.split(' ').map(|s| Segments::from_str(s).expect("Invalid digit in output")).collect()
    ))
}

impl AoCDay for Day08 {
    fn day(&self) -> usize {
        08
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("543"), Some("994266"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let data = parse_lines_with(input, parse)?;
        Ok(data.iter()
            .flat_map(|(_, o)| o)
            .map(Segments::count)
            .filter(|c| match c {
                // Use known segment count to map to digits
                // 1 | 4 | 7 | 8
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count()
            .to_string()
        )
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let data = parse_lines_with(input, parse)?;
        Ok(data.iter()
            .map(|(i, o)| deduce_digits(i, o))
            .map(|digits| digits.iter()
                .rev()
                .enumerate()
                .map(|(i, v)| v * 10_usize.pow(i as u32))
                .sum::<usize>()
            )
            .sum::<usize>()
            .to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day08)
}
