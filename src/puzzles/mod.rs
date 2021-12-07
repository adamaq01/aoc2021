#![allow(unused)]

use std::{collections::HashMap, error::Error, fmt::Display, str::FromStr};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Stage {
    First = 0,
    Second = 1,
}

impl Stage {
    pub fn new(stage: &str) -> std::result::Result<Self, String> {
        Self::from_str(stage)
    }
}

impl FromStr for Stage {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "0" | "first" => Ok(Stage::First),
            "1" | "second" => Ok(Stage::Second),
            _ => Err("Couldn't parse stage".into()),
        }
    }
}

pub trait Puzzle<I, O: Display>: Runnable {
    fn day(&self) -> u8;
    fn stage(&self) -> Stage;
    fn parse_inputs(&mut self, inputs: String) -> Result<I>;
    fn run(&mut self, inputs: I) -> Result<O>;
}

pub trait Runnable {
    fn fully_run(&mut self, inpust: String) -> Result<()>;
}

pub trait IntoPuzzle<I, O: Display, P: Puzzle<I, O>> {
    fn puzzle(self) -> P;
}

pub struct FnPuzzle<I, O: Display> {
    day: u8,
    stage: Stage,
    parse: Box<dyn FnMut(String) -> Result<I>>,
    func: Box<dyn FnMut(I) -> Result<O>>,
}

impl<I, O: Display> FnPuzzle<I, O> {
    pub fn new(
        day: u8,
        stage: Stage,
        parse: Box<dyn FnMut(String) -> Result<I>>,
        func: Box<dyn FnMut(I) -> Result<O>>,
    ) -> Self {
        Self {
            day,
            stage,
            parse,
            func,
        }
    }
}

impl<I, O: Display> Puzzle<I, O> for FnPuzzle<I, O> {
    fn day(&self) -> u8 {
        self.day
    }

    fn stage(&self) -> Stage {
        self.stage
    }

    fn parse_inputs(&mut self, inputs: String) -> Result<I> {
        (self.parse)(inputs)
    }

    fn run(&mut self, inputs: I) -> Result<O> {
        (self.func)(inputs)
    }
}

impl<I, O: Display> Runnable for FnPuzzle<I, O> {
    fn fully_run(&mut self, inputs: String) -> Result<()> {
        let inputs = self.parse_inputs(inputs)?;
        let output = self.run(inputs)?;

        println!("Solution: {}", output);

        Ok(())
    }
}

impl<I, O: Display, F> IntoPuzzle<I, O, FnPuzzle<I, O>> for F
where
    F: FnMut() -> FnPuzzle<I, O>,
{
    fn puzzle(mut self) -> FnPuzzle<I, O> {
        self()
    }
}

pub struct PuzzleRegistry {
    registry: HashMap<(u8, Stage), Box<dyn Runnable>>,
}

impl PuzzleRegistry {
    pub fn new() -> PuzzleRegistry {
        PuzzleRegistry {
            registry: HashMap::new(),
        }
    }

    pub fn register<I, O: Display, P: Puzzle<I, O> + 'static, T: IntoPuzzle<I, O, P>>(
        &mut self,
        puzzle: T,
    ) {
        let puzzle = puzzle.puzzle();
        let puzzle = Box::new(puzzle);
        self.registry.insert((puzzle.day(), puzzle.stage()), puzzle);
    }

    pub fn run(&mut self, day: u8, stage: Stage, inputs: String) -> Result<()> {
        if let Some(puzzle) = self.registry.get_mut(&(day, stage)) {
            puzzle.fully_run(inputs)?;
        }

        Ok(())
    }
}
