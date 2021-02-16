use std::u32::MAX;

pub fn part1(input: &[String]) -> u32 {
    let mut lines = input.iter();
    let earliest_timestamp = lines.next().unwrap().parse::<u32>().unwrap();
    let bus_ids = get_all_bus_ids(lines.next().unwrap());

    println!("all bus times: {:?}", bus_ids);

    let mut closest_bus_id = 0;
    let mut closest_diff = MAX;
    for bus_id in bus_ids.iter() {
        let bus_id_u32 = bus_id.parse::<u32>().unwrap();
        let curr_bus_earliest = get_bus_times_for_id(bus_id_u32, earliest_timestamp)
            .iter()
            .max()
            .unwrap()
            .clone();

        let curr_diff = curr_bus_earliest - earliest_timestamp;

        if curr_diff < closest_diff {
            closest_diff = curr_diff;
            closest_bus_id = bus_id_u32;
        }
    }

    closest_diff * closest_bus_id
}

fn get_all_bus_ids<T>(line: T) -> Vec<String>
where
    T: Into<String>,
{
    line.into()
        .split_terminator(",")
        .filter(|s| *s != "x")
        .map(|s| s.to_string())
        .collect()
}

fn get_bus_times_for_id(id: u32, earliest_timestamp: u32) -> Vec<u32> {
    let mut bus_times = Vec::new();
    let mut current_time = id;
    loop {
        bus_times.push(current_time);

        if current_time > earliest_timestamp {
            break;
        }

        current_time += id;
    }

    bus_times
}

#[cfg(test)]
mod tests {
    use std::u32::MAX;

    use super::{get_all_bus_ids, get_bus_times_for_id};

    #[test]
    fn test_get_part_1() {
        let earliest_timestamp = 939;
        let input = "7,13,x,x,59,x,31,19";

        let bus_ids = get_all_bus_ids(input);

        println!("all bus times: {:?}", bus_ids);

        let mut closest_bus_id = 0;
        let mut closest_diff = MAX;
        for bus_id in bus_ids.iter() {
            let bus_id_u32 = bus_id.parse::<u32>().unwrap();
            let curr_bus_earliest = get_bus_times_for_id(bus_id_u32, earliest_timestamp)
                .iter()
                .max()
                .unwrap()
                .clone();

            let curr_diff = curr_bus_earliest - earliest_timestamp;

            if curr_diff < closest_diff {
                closest_diff = curr_diff;
                closest_bus_id = bus_id_u32;
            }
        }

        assert_eq!(295, closest_bus_id * closest_diff);
    }
}
