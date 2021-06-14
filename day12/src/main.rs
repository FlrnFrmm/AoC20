use anyhow::Result;
use std::io::Read;

#[derive(Debug)]
enum Command {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64)
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, Clone, Copy)]
struct Position {
    horizontal: i64,
    vertical: i64
}

fn main() -> Result<()> {
    let commands = parse_input()?;

    println!("Result one: {:?}", p1(&commands)); 

    println!("Result two: {:?}", p2(&commands));

    Ok(())
}

fn p1(commands: &Vec<Command>) -> Result<usize> {
    let mut ship_direction = Direction::East;
    let mut ship_position = Position { horizontal:0, vertical:0 };

    commands.iter().for_each(|command|{
        match command {
            Command::North(value) => ship_position.vertical += value,
            Command::South(value) => ship_position.vertical -= value,
            Command::East(value) => ship_position.horizontal += value,
            Command::West(value) => ship_position.horizontal -= value,
            Command::Left(value) => {
                (0..value/90)
                    .for_each(|_| ship_direction = turn_ship_left(ship_direction));
            },
            Command::Right(value) => {
                (0..value/90)
                    .for_each(|_| ship_direction = turn_ship_right(ship_direction));
            },
            Command::Forward(value) => {
                match ship_direction {
                    Direction::North => ship_position.vertical += value,
                    Direction::East => ship_position.horizontal += value,
                    Direction::South => ship_position.vertical -= value,
                    Direction::West => ship_position.horizontal -= value
                }
            },
        };
    });

    Ok((ship_position.horizontal.abs() + ship_position.vertical.abs()) as usize)
}

fn p2(commands: &Vec<Command>) -> Result<usize> {
    let mut ship_position = Position { horizontal:0, vertical:0 };
    let mut waypoint_position = Position { horizontal:10, vertical:1 };

    commands.iter().for_each(|command|{
        match command {
            Command::North(value) => waypoint_position.vertical += *value,
            Command::South(value) => waypoint_position.vertical -= *value,
            Command::East(value) => waypoint_position.horizontal += *value,
            Command::West(value) => waypoint_position.horizontal -= *value,
            Command::Left(value) => {
                (0..value/90)
                    .for_each(|_| waypoint_position = turn_waypoint_left(waypoint_position));
            },
            Command::Right(value) => {
                (0..value/90)
                    .for_each(|_| waypoint_position = turn_waypoint_left(waypoint_position));
            },
            Command::Forward(value) => {
                ship_position.horizontal += value * waypoint_position.horizontal;
                ship_position.vertical += value * waypoint_position.vertical;
            },
        };
        println!("Ship => {:?}", ship_position);
        println!("Waypoint => {:?}\n", waypoint_position);
    });

    Ok((ship_position.horizontal.abs() + ship_position.vertical.abs()) as usize)
}

fn turn_ship_left(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North, 
        Direction::West => Direction::South, 
        Direction::South => Direction::East 
    }
}

fn turn_ship_right(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
        Direction::South => Direction::West
    }
}

fn turn_waypoint_left(waypoint_position: Position) -> Position {
    match (waypoint_position.horizontal > 0, waypoint_position.vertical > 0) {
        (false, false) => {
            Position { 
                horizontal: -waypoint_position.horizontal, 
                vertical: waypoint_position.vertical
            }
        },
        (true, false) => {
            Position { 
                horizontal: waypoint_position.horizontal, 
                vertical: -waypoint_position.vertical
            }
        },
        (true, true) => {
            Position { 
                horizontal: -waypoint_position.horizontal, 
                vertical: waypoint_position.vertical
            }
        },
        (false, true) => {
            Position { 
                horizontal: waypoint_position.horizontal, 
                vertical: -waypoint_position.vertical
            }
        }
    }
}

fn turn_waypoint_right(waypoint_position: Position) -> Position {
    match (waypoint_position.horizontal > 0, waypoint_position.vertical > 0) {
        (false, false) => {
            Position { 
                horizontal: waypoint_position.horizontal, 
                vertical: -waypoint_position.vertical
            }
        },
        (true, false) => {
            Position { 
                horizontal: -waypoint_position.horizontal, 
                vertical: waypoint_position.vertical
            }
        },
        (true, true) => {
            Position { 
                horizontal: waypoint_position.horizontal, 
                vertical: -waypoint_position.vertical
            }
        },
        (false, true) => {
            Position { 
                horizontal: -waypoint_position.horizontal, 
                vertical: waypoint_position.vertical
            }
        }
    }
}

fn parse_input() -> Result<Vec<Command>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    input.lines()
        .map(|s| {
            let number = s.chars().skip(1).collect::<String>().parse::<i64>()?;
            match s.chars().next().unwrap() {
                'N' => Ok(Command::North(number)),
                'S' => Ok(Command::South(number)),
                'E' => Ok(Command::East(number)),
                'W' => Ok(Command::West(number)),
                'L' => Ok(Command::Left(number)),
                'R' => Ok(Command::Right(number)),
                'F' => Ok(Command::Forward(number)),
                _ => Err(anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, "Couldn't parse input value.")))
            }
        })
        .collect()
}
