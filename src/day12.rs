use core::num;
use std::{fmt::Debug, str::FromStr};

pub fn part1(input: &[String]) -> usize {
    let actions: Vec<Action> = input
        .iter()
        .map(|line| Action::from_str(line).unwrap())
        .collect();

    let mut ship = Ship::new(Direction::East);

    actions.iter().for_each(|action| {
        ship.move_ship(action);
    });

    ship.manhattan_distance()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Movement {
    Forward,
    Left,
    Right,
}

enum Action {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl FromStr for Action {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.get(..1).unwrap();
        let distance = s
            .get(1..)
            .unwrap()
            .parse::<usize>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        match direction {
            "N" => Ok(Self::North(distance)),
            "E" => Ok(Self::East(distance)),
            "S" => Ok(Self::South(distance)),
            "W" => Ok(Self::West(distance)),
            "L" => Ok(Self::Left(distance)),
            "R" => Ok(Self::Right(distance)),
            "F" => Ok(Self::Forward(distance)),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid direction {}", v),
            )),
        }
    }
}

struct Ship {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Debug for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let up_down = if self.x >= 0 { "North" } else { "South" };
        let left_right = if self.y >= 0 { "East" } else { "West" };

        f.debug_struct("Ship")
            .field("direction", &self.direction)
            .field(up_down, &self.x.abs())
            .field(left_right, &self.y.abs())
            .finish()
    }
}

impl Ship {
    fn new(starting_direction: Direction) -> Self {
        Self {
            x: 0,
            y: 0,
            direction: starting_direction,
        }
    }

    fn move_ship(&mut self, action: &Action) {
        match action {
            Action::North(distance) => self.x += *distance as isize,
            Action::East(distance) => self.y += *distance as isize,
            Action::South(distance) => self.x -= *distance as isize,
            Action::West(distance) => self.y -= *distance as isize,
            Action::Forward(distance) => match self.direction {
                Direction::North => self.move_ship(&Action::North(*distance)),
                Direction::East => self.move_ship(&Action::East(*distance)),
                Direction::South => self.move_ship(&Action::South(*distance)),
                Direction::West => self.move_ship(&Action::West(*distance)),
            },
            Action::Left(angle) => self.change_direction(Movement::Left, *angle),
            Action::Right(angle) => self.change_direction(Movement::Right, *angle),
        }
    }

    fn change_direction(&mut self, movement: Movement, angle: usize) {
        let direction_ordering = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        // get index of current direction
        let mut direction_index = 0;
        for (i, d) in direction_ordering.iter().enumerate() {
            if self.direction == *d {
                direction_index = i;
                break;
            }
        }

        let num_direction_changes = ((angle / 90) % direction_ordering.len()) as isize;

        let new_direction_index: usize = match movement {
            Movement::Forward => {
                panic!("should not happen")
            }
            Movement::Left => {
                let mut i: isize = (direction_index as isize) - num_direction_changes;
                if i < 0 {
                    i = direction_ordering.len() as isize + i;
                }

                i as usize
            }
            Movement::Right => {
                (direction_index + (num_direction_changes as usize)) % direction_ordering.len()
            }
        };

        self.direction = direction_ordering[new_direction_index];
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

mod tests {
    use std::str::FromStr;

    use super::{Action, Ship};

    #[test]
    fn test_ship_movement() {
        let input = "F10
N3
F7
R90
F11";

        let actions: Vec<Action> = input
            .lines()
            .map(|line| Action::from_str(line).unwrap())
            .collect();

        let mut ship = Ship::new(super::Direction::East);

        println!("{:?}", ship);

        actions.iter().for_each(|action| {
            ship.move_ship(action);
            println!("{:?}", ship);
        });

        assert_eq!(ship.manhattan_distance(), 25);
    }
}
