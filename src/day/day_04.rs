use aoc_core::{AoCDay, ErrorWrapper, bail};

pub struct Day04;

type Num = u64;

#[derive(Debug, Clone, Default)]
struct Board {
    tiles: [Option<Num>; 25],
    win_call: Option<Num>,
    win_iter: Option<usize>,
}

impl Board {
    fn mark(&mut self, call: Num, iter: usize) -> bool {
        // Don't keep playing a board that has already bingo'd.
        if self.win_iter.is_none() {
            for i in 0..self.tiles.len() {
                let tile = self.tiles[i];
                if tile.is_some() && tile.as_ref().unwrap() == &call {
                    self.tiles[i] = None;
                }
            }
            let won = self.check_win();
            if won {
                self.win_call = Some(call);
                self.win_iter = Some(iter);
            }
            won
        } else {
            true
        }
    }
    fn check_win(&self) -> bool {
        for y in 0..5 {
            if self.tiles
                .iter()
                .enumerate()
                .filter(|(i, _t)| i % 5 == y)
                .all(|(_i, t)| t.is_none()) {
                return true;
            }
        }
        for x in 0..5 {
            if self.tiles[(x * 5)..(x * 5)+5].iter().all(|t| t.is_none()) {
                return true;
            }
        }
        false
    }
    fn score(&self) -> Option<Num> {
        if let Some(wc) = self.win_call {
            Some(self.tiles.iter()
                .filter_map(|t| *t)
                .sum::<Num>() * wc)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> (Vec<Num>, Vec<Board>) {
    let calls: Vec<Num> = input.lines()
        .nth(0)
        .expect("First line")
        .split(',')
        .map(|s| s.parse::<Num>().unwrap())
        .collect();

    let boards = input.split("\n\n").skip(1).filter(|s| !s.is_empty()).map(|board_str| {
        let mut board = Board::default();
        for (i, s) in board_str.split(|c| c == ' ' || c == '\n').filter(|s| !s.is_empty()).enumerate() {
            board.tiles[i] = Some(s.parse().expect("Invalid board"));
        }
        board
    }).collect();
    (calls, boards)
}

impl AoCDay for Day04 {
    fn day(&self) -> usize {
        04
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("22680"), Some("16168"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let (calls, mut boards) = parse(input);
        let mut winning_board = None;
        'calls: for (i, c) in calls.iter().enumerate() {
            for b in boards.iter_mut() {
                if b.mark(*c, i) {
                    winning_board = Some(b.clone());
                    break 'calls;
                }
            }
        }
        if let Some(board) = winning_board {
            Ok(board.score().unwrap().to_string())
        } else {
            bail!("Unable to find a winning board")
        }
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let (calls, mut boards) = parse(input);
        for (i, c) in calls.iter().enumerate() {
            for b in boards.iter_mut() {
                b.mark(*c, i);
            }
        }
        Ok(boards.iter()
            .filter(|b| b.win_iter.is_some())
            .max_by_key(|b| b.win_iter.unwrap())
            .expect("Unable to find last winning board")
            .score()
            .unwrap()
            .to_string())
        //Err(ErrorWrapper::NotImplemented)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day04)
}
