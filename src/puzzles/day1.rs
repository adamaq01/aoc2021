use super::{FnPuzzle, Result, Stage};

use std::str::FromStr;

fn parse_inputs(inputs: String) -> Result<Vec<u32>> {
    let mut numbers = Vec::new();
    for input in inputs.split("\n") {
        numbers.push(u32::from_str(input)?);
    }
    Ok(numbers)
}

#[puzzle(1, first, parse_inputs)]
pub fn first_stage(inputs: Vec<u32>) -> Result<usize> {
    Ok(inputs.windows(2).filter(|w| w[0] < w[1]).count())
}
