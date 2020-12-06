use anyhow::Result;
use std::io::Read;

fn main() -> Result<()> {
    let boarding_passes = parse_input()?;

    let occupied_seats: Vec<(usize,usize)> = boarding_passes.iter()
        .map(|pass| calculate_position(pass)).collect();

    let max_id_seat = calculate_max_id_seat(&occupied_seats);

    println!("Part one result: {:?} => {}", max_id_seat, max_id_seat.0 * 8 + max_id_seat.1);

    match calculate_my_seat(&occupied_seats) {
        Some(seat) => println!("Part two result: {:?} -> {}", seat, seat.0 * 8 + seat.1),
        None => println!("No free seat!")
    }

    Ok(())
}

fn calculate_position(code: &String) -> (usize, usize) {
    let row_part = code.chars().take(7).collect();
    let col_part = code.chars().skip(7).collect();

    let (row, col) = (calculate_row(row_part), calculate_col(col_part));

    (row, col)
}

fn calculate_row(code: Vec<char>) -> usize {
    let (position, _) = code.iter()
        .fold((0usize, 128usize), |(position, size), char|{
            let new_size = size / 2;
            if *char == 'F' {
                (position, new_size)
            } else {
                (position + new_size, new_size)
            }
        });
    position
}

fn calculate_col(code: Vec<char>) -> usize {
    let (position, _) = code.iter()
        .fold((0usize, 8usize), |(position, size), char|{
            let new_size = size / 2;
            if *char == 'L' {
                (position, new_size)
            } else {
                (position + new_size, new_size)
            }
        });
    position
}

fn calculate_max_id_seat(occupied_seats: &Vec<(usize, usize)>) -> (usize, usize) {
    occupied_seats.iter()
        .fold((std::usize::MIN, (0usize, 0usize)), 
         |(max_id, (max_row, max_col)), (row, col)| {
                let id = row * 8 + col;
                if id > max_id {
                    (id, (*row, *col))
                } else {
                    (max_id, (max_row, max_col))
                }
            })
        .1
}

fn calculate_my_seat(occupied_seats: &Vec<(usize, usize)>) -> Option<(usize, usize)> {
    (1..128)
        .map(move|r| (1..8).map(move |c| (r, c)))
        .flatten()
        .filter(|s| !occupied_seats.contains(s))
        .filter(|(r,c)| {
            occupied_seats.contains(&(r + 1, *c)) &&
            occupied_seats.contains(&(r - 1, *c)) &&
            occupied_seats.contains(&(*r, c + 1)) &&
            occupied_seats.contains(&(*r, c - 1)) &&
            occupied_seats.contains(&(r + 1, c + 1)) &&
            occupied_seats.contains(&(r - 1, c - 1)) &&
            occupied_seats.contains(&(r + 1, c - 1)) &&
            occupied_seats.contains(&(r - 1, c + 1))
        })
        .next()
}

fn parse_input() -> Result<Vec<String>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input.lines().map(|s| String::from(s)).collect())
}
