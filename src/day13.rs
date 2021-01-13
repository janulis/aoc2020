use crate::utils::stdin_to_vec;

fn get_earliest_bus_departure_time(earliest_daparture_time: usize, bus_id: usize) -> usize {
    if earliest_daparture_time % bus_id != 0 {
        earliest_daparture_time + bus_id - earliest_daparture_time % bus_id
    } else {
        earliest_daparture_time
    }
}

pub fn part1() {
    let input = stdin_to_vec::<String>();
    assert_eq!(2, input.len());

    let earliest_depart_time = input[0].parse::<usize>().unwrap();
    let bus_ids = input[1]
        .split(',')
        .filter(|id| *id != "x")
        .map(|id| id.parse::<usize>().expect("Could not parse bus id"))
        .collect::<Vec<usize>>();

    println!(
        "Earliest depart time: {}, Bus ids: {:?}",
        earliest_depart_time, bus_ids
    );

    let mut bus_departures = bus_ids
        .iter()
        .map(|bus_id| {
            (
                get_earliest_bus_departure_time(earliest_depart_time, *bus_id),
                *bus_id,
            )
        })
        .collect::<Vec<(usize, usize)>>();

    bus_departures.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("Earliest bus departures: {:?}", bus_departures);

    if let Some(earliest_bus_departure) = bus_departures.first() {
        let minutes_to_wait = earliest_bus_departure.0 - earliest_depart_time;
        let bus_id = earliest_bus_departure.1;
        println!("Result: {}", minutes_to_wait * bus_id);
    }
}

fn get_bus_offsets(bus_id_str: &str) -> Vec<(usize, usize)> {
    let bus_id_strs = bus_id_str.split(',').collect::<Vec<&str>>();

    let mut bus_offsets: Vec<(usize, usize)> = vec![];

    for (i, bus_id_str) in bus_id_strs.iter().enumerate() {
        if *bus_id_str == "x" {
            continue;
        }

        let bus_id = bus_id_str.parse::<usize>().expect("Could not parse bus id");
        bus_offsets.push((bus_id, i));
    }

    bus_offsets
}

fn get_earliest_time(input: &str) -> usize {
    let bus_id_offsets = get_bus_offsets(input);
    println!("{:?}", bus_id_offsets);

    let first_bus_id = bus_id_offsets.first().unwrap().0;
    let mut first_bus_departure_time = first_bus_id;

    loop {
        let mut found_time = true;
        for i in 1..bus_id_offsets.len() {
            let bus_id = bus_id_offsets[i].0;
            let bus_time_offset = bus_id_offsets[i].1;
            
            if (first_bus_departure_time + bus_time_offset) % bus_id != 0 {
                found_time = false;
                break;
            }
        }

        if found_time {
            return first_bus_departure_time;
        }

        first_bus_departure_time += first_bus_id;
    }
}

#[test]
fn test_get_earliest_time() {
    assert_eq!(get_earliest_time("7,13,x,x,59,x,31,19"), 1068781);
    assert_eq!(get_earliest_time("17,x,13,19"), 3417);
    assert_eq!(get_earliest_time("67,7,59,61"), 754018);
    assert_eq!(get_earliest_time("67,x,7,59,61"), 779210);
    assert_eq!(get_earliest_time("67,7,x,59,61"), 1261476);
    assert_eq!(get_earliest_time("1789,37,47,1889"), 1202161486);
}

pub fn part2() {
    let input = stdin_to_vec::<String>();
    assert_eq!(2, input.len());
    println!("Earliest timestamp: {}", get_earliest_time(&input[1]));
}
