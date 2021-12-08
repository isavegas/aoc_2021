use aoc_core::{AoCDay, ErrorWrapper, parse_lines, Vec2};
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day05;

type Num = i64;

#[derive(Debug, Clone)]
struct Segment {
    start: Vec2<Num>,
    end: Vec2<Num>,
}

#[derive(Debug, Clone)]
struct SegmentIterator<'a> {
    segment: &'a Segment,
    index: usize,
    done: bool,
}

fn clamp_one(v: Num) -> Num {
    if v.is_positive() {
        std::cmp::min(v, 1)
    } else {
        -std::cmp::min(v.abs(), 1)
    }
}

impl Iterator for SegmentIterator<'_> {
    type Item = Vec2<Num>;
    fn next(&mut self) -> Option<Vec2<Num>> {
        if self.done { return None; }
        // aligned implementation atm
        let dir = Vec2::<Num>::new(
            clamp_one(self.segment.end.x - self.segment.start.x),
            clamp_one(self.segment.end.y - self.segment.start.y),
        );
        let n = self.segment.start + (dir * Vec2::<Num>::new(self.index as Num, self.index as Num));
        if self.segment.end != n {
            self.index += 1;
            Some(n)
        } else {
            self.done = true;
            Some(n)
        }
    }
}

impl Segment {
    fn new(start: Vec2<Num>, end: Vec2<Num>) -> Self {
        Self {
            start,
            end,
        }
    }
    fn points(&self) -> SegmentIterator {
        SegmentIterator {
            segment: self,
            index: 0,
            done: false,
        }
    }
}

impl FromStr for Segment {
    type Err = ErrorWrapper;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut comps = s.split(" -> ");
        let start = comps.next()
            .ok_or_else(|| ErrorWrapper::ParseError("Missing start".to_string()))?
            .parse()?;
        let end = comps.next()
            .ok_or_else(|| ErrorWrapper::ParseError("Missing start".to_string()))?
            .parse()?;
        if comps.next().is_none() {
            Ok(Self::new(
                start,
                end,
            ))
        } else {
            Err(ErrorWrapper::ParseError("Too many components".to_string()))
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Map {
    tiles: HashMap<Vec2<Num>, Num>,
}

impl Map {
    fn overlay(&mut self, segment: &Segment, aligned: bool) {
        if aligned && segment.start.x != segment.end.x && segment.start.y != segment.end.y {
            return;
        }
        for p in segment.points() {
            *self.tiles.entry(p).or_insert(0) += 1;
        }
    }
}

impl AoCDay for Day05 {
    fn day(&self) -> usize {
        05
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("5698"), Some("15463"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let vents: Vec<Segment> = parse_lines(input)?;
        let mut map = Map::default();
        for v in &vents {
            map.overlay(v, true);
        }
        Ok(map.tiles.values()
            .filter(|v| **v >= 2)
            .count().to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let vents: Vec<Segment> = parse_lines(input)?;
        let mut map = Map::default();
        for v in &vents {
            map.overlay(v, false);
        }
        Ok(map.tiles.values()
            .filter(|v| **v >= 2)
            .count().to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day05)
}
