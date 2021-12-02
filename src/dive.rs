use crate::prelude::*;
use std::convert::{TryFrom, TryInto};

pub(crate) async fn eval(input: Vec<String>) -> Result<()> {
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq)]
struct Command {
    direction: Direction,
    distance: i32,
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err("unknown direction".to_string()),
        }
    }
}
impl TryFrom<&str> for Command {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s = value.split(" ").collect::<Vec<&str>>();
        if s.len() != 2 {
            return Err("line is incorrectly formatted".to_string());
        }

        let direction: Direction = s[0].try_into()?;
        let distance = s[1]
            .parse::<i32>()
            .or_else(|e| Err("can't parse distance".to_string()))?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

fn parse_input(input: Vec<String>) -> Vec<Command> {
    input.iter().map(|elem| elem.as_str().try_into()?).collect()
}
