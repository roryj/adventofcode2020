use std::{fmt::Debug, str::FromStr};

pub fn part1(input: &[String]) -> usize {
    let mut seats = GameOfSeatingLife::new(input);

    let mut count = 1;
    loop {
        // println!("tick #{}", count);
        if seats.tick() == 0 {
            break;
        }

        // println!("map:\n{:?}", map);
        count += 1;
    }

    seats.get_num_occupied_seats()
}

struct GameOfSeatingLife {
    position: Vec<Vec<FerryPosition>>,
}

impl Debug for GameOfSeatingLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        self.position.iter().for_each(|line| {
            line.iter().for_each(|p| {
                result = format!("{}{:?} ", result, p);
            });
            result = format!("{}\n", result);
        });
        f.write_str(&result)
    }
}

impl GameOfSeatingLife {
    pub fn new(input: &[String]) -> Self {
        let start_map = input
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| FerryPosition::from_str(&c.to_string()).unwrap())
                    .collect()
            })
            .collect();

        Self {
            position: start_map,
        }
    }

    // for every tick, we must apply the following rules:
    // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    // Otherwise, the seat's state does not change.
    pub fn tick(&mut self) -> usize {
        let mut new_ferry_map = self.position.clone();
        let mut num_state_changes = 0;

        self.position
            .iter()
            .enumerate()
            .for_each(|(row_index, line)| {
                line.iter().enumerate().for_each(|(column_index, p)| {
                    // the main logic
                    match p {
                        FerryPosition::EmptySeat => {
                            // seat is empty
                            let adjacent_occupied_seats: Vec<FerryPosition> = self
                                .get_adjacent_states(row_index as isize, column_index as isize)
                                .iter()
                                .filter_map(|p| {
                                    if *p != FerryPosition::OccupiedSeat {
                                        return None;
                                    }

                                    Some(p.to_owned())
                                })
                                .collect();

                            // println!(
                            //     "adjacent occupied seats @ {},{}: {:?}",
                            //     row_index, column_index, adjacent_occupied_seats
                            // );
                            if adjacent_occupied_seats.len() > 0 {
                                return;
                            }

                            // all unoccupied! Set is occupied
                            // println!(
                            //     "swapping @ {},{} to {:?}",
                            //     row_index,
                            //     column_index,
                            //     FerryPosition::OccupiedSeat
                            // );
                            num_state_changes += 1;
                            new_ferry_map[row_index][column_index] = FerryPosition::OccupiedSeat
                        }
                        FerryPosition::OccupiedSeat => {
                            let adjacent_occupied_seats: Vec<FerryPosition> = self
                                .get_adjacent_states(row_index as isize, column_index as isize)
                                .iter()
                                .filter_map(|p| {
                                    if *p != FerryPosition::OccupiedSeat {
                                        return None;
                                    }

                                    Some(p.to_owned())
                                })
                                .collect();
                            // println!(
                            //     "adjacent occupied seats @ {},{}: {:?}",
                            //     row_index, column_index, adjacent_occupied_seats
                            // );
                            if adjacent_occupied_seats.len() < 4 {
                                return;
                            }

                            // four are occupied! Seat is now empty
                            num_state_changes += 1;
                            new_ferry_map[row_index][column_index] = FerryPosition::EmptySeat
                        }
                        FerryPosition::Floor => {}
                    };
                })
            });

        // println!("new map: {:?}", new_ferry_map);

        self.position = new_ferry_map;
        num_state_changes
    }

    fn get_adjacent_states(&self, row_index: isize, column_index: isize) -> Vec<FerryPosition> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .filter_map(|(row_delta, column_delta)| {
            if !self.is_in_bounds(
                (row_index as isize) + row_delta,
                (column_index as isize) + column_delta,
            ) {
                return None;
            }
            Some(self.get_state_at_position(
                (row_index + row_delta) as usize,
                (column_index + column_delta) as usize,
            ))
        })
        .collect()
    }

    fn get_num_occupied_seats(&self) -> usize {
        self.position
            .iter()
            .flatten()
            .filter(|p| **p == FerryPosition::OccupiedSeat)
            .count()
    }

    fn is_in_bounds(&self, row_index: isize, column_index: isize) -> bool {
        row_index >= 0
            && (row_index as usize) < self.position.len()
            && column_index >= 0
            && (column_index as usize) < self.position[0].len()
    }

    fn get_state_at_position(&self, row_index: usize, column_index: usize) -> FerryPosition {
        self.position
            .get(row_index)
            .unwrap()
            .get(column_index)
            .unwrap()
            .to_owned()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FerryPosition {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl Debug for FerryPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FerryPosition::OccupiedSeat => "#",
            FerryPosition::EmptySeat => "L",
            FerryPosition::Floor => ".",
        })
    }
}

impl FromStr for FerryPosition {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Floor),
            "L" => Ok(Self::EmptySeat),
            "#" => Ok(Self::OccupiedSeat),
            v => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("invalid input: {:?}. Expected one of [L, ., #]", v),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameOfSeatingLife;

    #[test]
    fn game_of_seating_life_test() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

        let mut map = GameOfSeatingLife::new(&lines);

        let mut count = 1;
        loop {
            println!("tick #{}", count);
            if map.tick() == 0 {
                break;
            }

            println!("map:\n{:?}", map);
            count += 1;
        }

        assert_eq!(37, map.get_num_occupied_seats());
    }
}
