use aoc_core::{AoCDay, ErrorWrapper};
use std::str::FromStr;

pub struct Day09;

type Num = u64;

#[derive(Debug, Clone, Default)]
struct Map {
    height: usize,
    width: usize,
    data: Vec<Num>,
}

#[inline(always)]
fn apply_offset(value: usize, offset: isize) -> Option<usize> {
    if offset.is_positive() || offset == 0 {
        Some(value + offset as usize)
    } else {
        let o = offset.abs() as usize;
        // Cannot offset below zero
        if o > value {
            None
        } else {
            Some(value - o)
        }
    }
}

#[inline(always)]
fn offsets(width: usize, v: usize) -> [Option<usize>;4] {
    let w = width as isize;
    let mut offsets = [None; 4];

    // x == anything
    offsets[0] = apply_offset(v, -w);
    offsets[1] = apply_offset(v, w);

    // x != width - 1
    if v % width != width - 1 {
        offsets[2] = apply_offset(v, 1);
    }
    // x != 0
    if v % width != 0 {
        offsets[3] = apply_offset(v, -1);
    }

    offsets
}

impl Map {
    fn find_basins(&self) -> Result<Vec<usize>, ErrorWrapper> {
        let mut basins: Vec<Vec<usize>> = vec![];
        for (i, v) in self.data.iter().enumerate() {
            if *v != 9 {
                
            }
        }
        Ok(basins.iter().map(|b| b.len()).collect())
    }
    fn find_lows(&self) -> Result<Vec<Num>, ErrorWrapper> {
        // Get offsets from current point
        Ok(self.data.iter()
            .enumerate()
            .filter_map(|(i, v)| {
                let mut low = true;
                // filter_map on identity to iterate over Some(o) values
                for o in offsets(self.width, i).iter()
                    .filter_map(|o| o.as_ref())
                {
                    if let Some(other) = self.data.get(*o) {
                        if v >= other {
                            low = false;
                            break;
                        }
                    }
                };
                if low {
                    Some(*v)
                } else {
                    None
                }
            })
            .collect())
    }
}

impl FromStr for Map {
    type Err = ErrorWrapper;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut map = Map::default();
        for line in str.lines() {
            if line.len() > map.width {
                map.width = line.len();
            }
            map.height += 1;
            for i in line.chars().map(|c| char::to_digit(c, 10)) {
                map.data.push(i.ok_or_else(|| ErrorWrapper::Simple("Invalid height".to_string()))?.into());
            }
        }
        Ok(map)
    }
}

impl AoCDay for Day09 {
    fn day(&self) -> usize {
        09
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("526"), None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let map: Map = Map::from_str(input.trim())?;
        Ok(map.find_lows()?.iter().map(|v| v + 1).sum::<Num>().to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let map: Map = Map::from_str(input.trim())?;
        let mut basins = map.find_basins()?;
        basins.sort();
        Ok(basins.iter().take(3).product::<usize>().to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day09)
}
