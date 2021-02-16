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

pub fn part2(input: &[String]) -> usize {
    let actions: Vec<Action> = input
        .iter()
        .map(|line| Action::from_str(line).unwrap())
        .collect();

    let mut ship = Ship::new_with_waypoint_location(Direction::East, 10, 1);

    actions.iter().for_each(|action| {
        ship.move_waypoint_and_ship(action);

        match action {
            Action::Forward(d) => println!("moving forward {}", d),
            Action::Left(d) => println!("rotating wavepoint left {}", d),
            Action::Right(d) => println!("rotating wavepoint right {}", d),
            _ => {}
        }
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

#[derive(Debug)]
enum Movement {
    Left,
    Right,
}

#[derive(Debug)]
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
    waypoint_x: isize,
    waypoint_y: isize,
    direction: Direction,
}

impl Debug for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let up_down = if self.y >= 0 { "North" } else { "South" };
        let left_right = if self.x >= 0 { "East" } else { "West" };
        let waypoint_up_down = if self.waypoint_y >= 0 {
            "North"
        } else {
            "South"
        };
        let waypoint_left_right = if self.waypoint_x >= 0 { "East" } else { "West" };

        f.debug_struct("Ship")
            .field("facing", &self.direction)
            .field(up_down, &self.y.abs())
            .field(left_right, &self.x.abs())
            .field(
                "Waypoint (relative to ship)",
                &format!(
                    "{}->{:?}, {}->{:?}",
                    waypoint_up_down,
                    self.waypoint_y.abs(),
                    waypoint_left_right,
                    self.waypoint_x.abs()
                ),
            )
            .finish()
    }
}

impl Ship {
    fn new(starting_direction: Direction) -> Self {
        Self {
            x: 0,
            y: 0,
            waypoint_x: 0,
            waypoint_y: 0,
            direction: starting_direction,
        }
    }
    fn new_with_waypoint_location(
        starting_direction: Direction,
        waypoint_x: isize,
        waypoint_y: isize,
    ) -> Self {
        Self {
            x: 0,
            y: 0,
            waypoint_x,
            waypoint_y,
            direction: starting_direction,
        }
    }

    fn move_ship(&mut self, action: &Action) {
        match action {
            Action::North(distance) => self.y += *distance as isize,
            Action::East(distance) => self.x += *distance as isize,
            Action::South(distance) => self.y -= *distance as isize,
            Action::West(distance) => self.x -= *distance as isize,
            Action::Forward(distance) => match self.direction {
                Direction::North => self.move_ship(&Action::North(*distance)),
                Direction::East => self.move_ship(&Action::East(*distance)),
                Direction::South => self.move_ship(&Action::South(*distance)),
                Direction::West => self.move_ship(&Action::West(*distance)),
            },
            Action::Left(angle) => self.change_ship_direction(Movement::Left, *angle),
            Action::Right(angle) => self.change_ship_direction(Movement::Right, *angle),
        }
    }

    fn move_waypoint_and_ship(&mut self, action: &Action) {
        match action {
            Action::North(distance) => self.waypoint_y += *distance as isize,
            Action::East(distance) => self.waypoint_x += *distance as isize,
            Action::South(distance) => self.waypoint_y -= *distance as isize,
            Action::West(distance) => self.waypoint_x -= *distance as isize,
            Action::Forward(distance) => self.move_to_wavepoint(*distance),
            Action::Left(angle) => self.rotate_wavepoint(Movement::Left, *angle),
            Action::Right(angle) => self.rotate_wavepoint(Movement::Right, *angle),
        }
    }

    fn move_to_wavepoint(&mut self, distance: usize) {
        // find distance x and y to waypoint from ship
        // println!("attempting moving to wavepoint!");
        // println!("wavepoint: ({}, {})", self.waypoint_x, self.waypoint_y);
        // println!("ship location: ({}, {})", self.x, self.y);

        let x_change = self.waypoint_x * (distance as isize);
        let y_change = self.waypoint_y * (distance as isize);

        self.x += x_change;
        self.y += y_change;
        // println!("new ship location: ({}, {})", self.x, self.y);
    }

    fn rotate_wavepoint(&mut self, movement: Movement, angle: usize) {
        let angle = match movement {
            Movement::Right => angle,
            Movement::Left => 360 - angle,
            other => panic!("invalid movement {:?}", other),
        };

        let (new_waypoint_x, new_waypoint_y) = match angle {
            x if x == 90 => (self.waypoint_y, -self.waypoint_x),
            x if x == 180 => (-self.waypoint_x, -self.waypoint_y),
            x if x == 270 => (-self.waypoint_y, self.waypoint_x),
            x => panic!("invalid angle {}", x),
        };

        // println!("x change {} y change {}", new_waypoint_x, new_waypoint_y);

        self.waypoint_x = new_waypoint_x;
        self.waypoint_y = new_waypoint_y;
    }

    fn change_ship_direction(&mut self, movement: Movement, angle: usize) {
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

#[cfg(test)]
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

    #[test]
    fn test_ship_movement_part_2() {
        let input = "F10
N3
F7
R90
F11";

        let actions: Vec<Action> = input
            .lines()
            .map(|line| Action::from_str(line).unwrap())
            .collect();

        let mut ship = Ship::new_with_waypoint_location(super::Direction::East, 10, 1);

        println!("{:?}", ship);

        actions.iter().for_each(|action| {
            println!("action: {:?}", action);
            ship.move_waypoint_and_ship(action);
            println!("{:?}", ship);
        });

        assert_eq!(ship.manhattan_distance(), 286);
    }
}
