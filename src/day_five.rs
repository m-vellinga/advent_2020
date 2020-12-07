pub fn run(input: String) {
    let boarding_passes: Vec<&str> = input.lines().collect();

    println!("Day 5 Part 1: {}", part_1(&boarding_passes));
    println!("Day 5 Part 2: {}", part_2(&boarding_passes));
}

fn part_1(boarding_passes: &Vec<&str>) -> u64 {
    let mut highest_seat_id: u64 = 0;

    for pass in boarding_passes {
        let row = determine_row(&pass);
        let column = determine_column(&pass);
        let seat_id = calculate_seat_id(row as u64, column as u64);

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    highest_seat_id
}

fn part_2(boarding_passes: &Vec<&str>) -> u64 {
    let mut seat_ids: Vec<u64> = Vec::new();

    for pass in boarding_passes {
        let row = determine_row(&pass);
        let column = determine_column(&pass);
        let seat_id = calculate_seat_id(row as u64, column as u64);
        seat_ids.push(seat_id);
    }

    seat_ids.sort();

    for (i, seat_id) in seat_ids.iter().enumerate() {
        let expected_next_seat = seat_id + 1;
        if expected_next_seat != seat_ids[i + 1] {
            return expected_next_seat;
        }
    }

    panic!("Not found!");
}

fn determine_row(row_input: &str) -> u8 {
    let mut upper: u8 = 127;
    let mut lower: u8 = 0;
    for character in row_input[..7].chars() {
        let values = upper - lower + 1;
        if character == 'F' {
            // Lower half.
            upper = lower + ((values / 2) - 1);
        } else if character == 'B' {
            // Upper half.
            lower = upper - ((values / 2) - 1);
        }
    }
    lower
}

fn determine_column(column_input: &str) -> u8 {
    let mut upper: u8 = 7;
    let mut lower: u8 = 0;
    for character in column_input[7..].chars() {
        let values = upper - lower + 1;
        if character == 'L' {
            // Lower half.
            upper = lower + ((values / 2) - 1);
        } else if character == 'R' {
            // Upper half.
            lower = upper - ((values / 2) - 1);
        }
    }
    lower
}

fn calculate_seat_id(row: u64, column: u64) -> u64 {
    (row * 8) + column
}

#[test]
fn first_input() {
    let row = determine_row("BFFFBBFRRR");
    let column = determine_column("BFFFBBFRRR");
    let seat_id = calculate_seat_id(row as u64, column as u64);
    assert_eq!(row, 70);
    assert_eq!(column, 7);
    assert_eq!(seat_id, 567);
}
#[test]
fn second_input() {
    let row = determine_row("FFFBBBFRRR");
    let column = determine_column("FFFBBBFRRR");
    let seat_id = calculate_seat_id(row as u64, column as u64);
    assert_eq!(row, 14);
    assert_eq!(column, 7);
    assert_eq!(seat_id, 119);
}

#[test]
fn third_input() {
    let row = determine_row("BBFFBBFRLL");
    let column = determine_column("BBFFBBFRLL");
    let seat_id = calculate_seat_id(row as u64, column as u64);
    assert_eq!(row, 102);
    assert_eq!(column, 4);
    assert_eq!(seat_id, 820);
}
