use aoc_core::{AoCDay, ErrorWrapper, parse};
use std::str::FromStr;

pub struct Day02;

type Num = u64;

#[derive(Debug,Clone)]
enum Direction {
    Forward,
    Down,
    Up,
}
impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err("Unknown direction"),
        }
    }
}

#[derive(Debug,Clone)]
struct Instruction {
    direction: Direction,
    distance: Num,
}
impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let comps = s.split(' ').collect::<Vec<&str>>();
        if comps.len() != 2 {
            Err("Invalid instruction")
        } else {
            let dir = Direction::from_str(comps[0]);
            let dist = comps[1].parse::<Num>();
            if dir.is_ok() && dist.is_ok() {
                Ok(Self {
                    direction: dir.unwrap(),
                    distance: dist.unwrap(),
                })
            } else {
                Err("Invalid instruction")
            }
        }
    }
}

impl AoCDay for Day02 {
    fn day(&self) -> usize {
        02
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("1654760"), Some("1956047400"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let instructions = parse::<Instruction>(input);
        let mut distance = 0;
        let mut depth = 0;
        for i in instructions.iter() {
            match i.direction {
                Direction::Forward => distance += i.distance,
                Direction::Down => depth += i.distance,
                Direction::Up => depth -= i.distance,
            }
        }
        Ok((distance * depth).to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let instructions = parse::<Instruction>(input);
        let mut distance: Num = 0;
        let mut depth: Num = 0;
        let mut aim: Num = 0;
        for i in instructions.iter() {
            match i.direction {
                Direction::Forward => {
                    distance += i.distance;
                    depth += aim * i.distance;
                },
                Direction::Down => aim += i.distance,
                Direction::Up => aim -= i.distance,
            }
        }
        Ok((distance * depth).to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day02)
}
