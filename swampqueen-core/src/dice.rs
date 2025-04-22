use std::{fmt::Display, sync::OnceLock};

use color_eyre::eyre::{self, eyre};
use rand;
use regex::Regex;

static ROLL_REG: OnceLock<Regex> = OnceLock::new();

fn get_roll_reg() -> &'static Regex {
    ROLL_REG.get_or_init(|| Regex::new(r"^(\d+)d(\d+)$").unwrap())
}

/// A request to roll a given number of dice, each with a given number of sides.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct RollManyRequest {
    num_dice: usize,
    num_faces: usize,
}

/// A set of results from a group of rolled dice stemming from a [`RollManyRequest`].
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RolledDiceResults {
    request: RollManyRequest,
    results: Vec<usize>,
}
impl Display for RolledDiceResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.results.len() {
            0 => {
                write!(f, "0")
            }
            1 => {
                write!(f, "{}", self.results[0])
            }
            _ => {
                let results_str = self.results.iter().fold(String::new(), |acc, &num| {
                    if acc.is_empty() {
                        num.to_string()
                    } else {
                        acc + "+" + &num.to_string()
                    }
                });
                let results_sum: usize = self.results.iter().sum();
                write!(f, "{}={}", results_str, results_sum)
            }
        }
    }
}

/// Take a string of format `XdY`, where
///
/// - `X` is the number of dice being rolled.
/// - `Y` is the number of faces the dice have.
///
/// Returns [`RolledDiceResults`], representing the numerical values of all the rolled dice and the
/// parsed request.
///
/// # Examples
///
/// ```
/// // Roll three six-sided dice
/// let results = roll_many_from_str("3d6");
/// // Roll a d20
/// let d20_result = roll_many_from_str("1d20");
/// ```
///
/// # Errors
///
/// This function returns [`Err`] if the `input_str` arg is the incorrect format.
pub fn roll_many_from_str(input_str: &str) -> eyre::Result<RolledDiceResults> {
    let request = parse_roll_many_str(input_str)?;
    let results = roll_many(request);
    Ok(RolledDiceResults { request, results })
}

/// Take a string and, if it's the correct format, return a [`RollManyRequest`].
fn parse_roll_many_str(input_str: &str) -> eyre::Result<RollManyRequest> {
    let captures = get_roll_reg()
        .captures(input_str)
        .ok_or_else(|| eyre!("invalid input"))?;

    let num_dice = captures[1].parse()?;
    let num_faces = captures[2].parse()?;

    Ok(RollManyRequest {
        num_dice,
        num_faces,
    })
}

/// The function "roll_die" takes in a single `usize` as an argument.
/// That `usize` represents how many faces the dice has.
/// It returns a single `usize` that represents the number that was rolled.
pub fn roll_die(num_faces: usize) -> usize {
    // Set the "result" variable (which has data type "usize") to a random number from 1 to "num_faces"
    // (inclusive)
    let result: usize = rand::random_range(1..=num_faces);
    // Return the random number you rolled!
    return result;
}

/// Roll a given number of dice with the given number of faces, both corresponding to the provided
/// [`RollManyRequest`].
pub fn roll_many(request: RollManyRequest) -> Vec<usize> {
    (0..request.num_dice)
        .map(|_| roll_die(request.num_faces))
        .collect()
}
