use std::str::Chars;

pub fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|ticket_deets| {
            let row = calculate_row(&ticket_deets[..7], 128);
            let column = calculate_column(&ticket_deets[7..], 8);

            (row * 8) + column
        })
        .max()
        .unwrap()
}

#[derive(Clone, Debug)]
struct TicketData {
    row: u32,
    column: u32,
    id: u32,
}

impl Ord for TicketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for TicketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TicketData {}

impl PartialEq for TicketData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub fn part2(input: &[String]) -> u32 {
    let mut all_ids: Vec<TicketData> = input
        .iter()
        .map(|ticket_deets| {
            let row = calculate_row(&ticket_deets[..7], 128);
            let column = calculate_column(&ticket_deets[7..], 8);

            let id = (row * 8) + column;
            TicketData { row, column, id }
        })
        .collect();

    all_ids.sort();

    all_ids.iter().for_each(|i| println!("id: {:?}", i));

    let mut prev = 0;
    for ticket in all_ids.clone() {
        if prev == 0 {
            prev = ticket.id;
            continue;
        }

        if prev == (ticket.id - 1) {
            prev = ticket.id;
            continue;
        }

        // this is it???
        return ticket.id - 1;
    }

    panic!("this shouldnt be hit")
}

fn calculate_row(input: &str, row_count: usize) -> u32 {
    assert_eq!(input.len(), 7);

    let mut possible_rows: Vec<usize> = (0..row_count).collect();

    binary_search(input.chars(), &mut possible_rows)
}

fn calculate_column(input: &str, column_count: usize) -> u32 {
    assert_eq!(input.len(), 3);

    let mut possible_columns: Vec<usize> = (0..column_count).collect();

    binary_search(input.chars(), &mut possible_columns)
}

fn binary_search(mut chars: Chars, possible_seat: &mut [usize]) -> u32 {
    let max = possible_seat.len();
    let min = 0;
    let mut mid = min + ((max - min) / 2);
    mid = ((mid as f32).floor()) as usize;

    // println!("chars: {:?}", chars);
    // println!("rows: {:?}", possible_seat);
    // println!("min: {:?} mid: {:?} max: {:?}", min, mid, max);

    assert!(max > 0);

    let c = chars.next();
    if c.is_none() {
        // println!("no more chars!");
        assert_eq!(possible_seat.len(), 1);
        return possible_seat[0] as u32;
    }

    match c.unwrap() {
        'F' => binary_search(chars, &mut possible_seat[min..mid]),
        'B' => binary_search(chars, &mut possible_seat[mid..max]),
        'L' => binary_search(chars, &mut possible_seat[min..mid]),
        'R' => binary_search(chars, &mut possible_seat[mid..max]),
        unexpected => panic!("unexpected character: {:?}", unexpected),
    }
}

#[cfg(test)]
mod tests {
    use super::{calculate_column, calculate_row};

    #[test]
    fn calulate_row_happy() {
        let ticket_data = "FBFBBFFRLR";

        let result = calculate_row(&ticket_data[..7], 128);

        assert_eq!(result, 44);
    }

    #[test]
    fn calculate_column_happy() {
        let ticket_data = "FBFBBFFRLR";

        let column = calculate_column(&ticket_data[7..], 8);

        assert_eq!(column, 5);
    }
}
