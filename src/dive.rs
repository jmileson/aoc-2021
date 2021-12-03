use std::convert::{TryFrom, TryInto};

#[derive(Default, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
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
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(anyhow::anyhow!("unknown direction")),
        }
    }
}
impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s = value.split(" ").collect::<Vec<&str>>();
        if s.len() != 2 {
            return Err(anyhow::anyhow!("line is incorrectly formatted"));
        }

        let direction: Direction = s[0].try_into()?;
        let distance = s[1]
            .parse::<i32>()
            .or_else(|_| Err(anyhow::anyhow!("can't parse distance")))?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

impl Position {
    fn dist(&self) -> i32 {
        self.x * self.y
    }
}

pub(crate) async fn eval(input: Vec<String>) -> Result<(), anyhow::Error> {
    let commands = parse_input(input)?;
    let pos = compute(commands);

    println!("Position is: {:#?}", pos);
    println!("Distance is: {:#?}", pos.dist());
    Ok(())
}

fn parse_input(input: Vec<String>) -> Result<Vec<Command>, anyhow::Error> {
    input.iter().map(|elem| elem.as_str().try_into()).collect()
}

fn compute(commands: Vec<Command>) -> Position {
    let mut pos = Position::default();

    for command in commands.iter() {
        match command.direction {
            Direction::Forward => {
                pos.x += command.distance;
                pos.y += command.distance * pos.aim;
            }
            Direction::Down => pos.aim += command.distance,
            Direction::Up => pos.aim -= command.distance,
        }
    }

    pos
}

#[cfg(test)]
mod test {
    use super::{compute, Command, Direction, Position};

    #[test]
    fn test_compute() {
        let commands = vec![
            Command {
                direction: Direction::Forward,
                distance: 5,
            },
            Command {
                direction: Direction::Down,
                distance: 5,
            },
            Command {
                direction: Direction::Forward,
                distance: 8,
            },
            Command {
                direction: Direction::Up,
                distance: 3,
            },
            Command {
                direction: Direction::Down,
                distance: 8,
            },
            Command {
                direction: Direction::Forward,
                distance: 2,
            },
        ];
        let pos = compute(commands);

        assert_eq!(
            Position {
                x: 15,
                y: 60,
                aim: 10
            },
            pos
        );
    }

    #[test]
    fn test_dist() {
        let pos = Position {
            x: 15,
            y: 60,
            aim: 0,
        };

        assert_eq!(900, pos.dist());
    }
}
