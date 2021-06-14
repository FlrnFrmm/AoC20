use anyhow::Result;
use std::io::Read;
use std::collections::HashMap;

fn main() -> Result<()> {
    let room = parse_input()?;

    println!("Result one: {:?}", part_one(&room)); 

    println!("Result two: {:?}", part_two(&room));

    Ok(())
}

fn part_one(room: &Vec<Vec<char>>) -> Result<usize> {
    let mut map: HashMap<(i64, i64), char> = (0..room.len())
            .map(|row_index| (0..room[row_index].len()).map(move |col_index| ((row_index as i64, col_index as i64), room[row_index][col_index])))
            .flatten()
            .collect();

    while let Some(next_map) = next_map_part_one(&map) {
        map = next_map;
    }

    Ok(map.iter().filter(|(_, v)| **v == '#').count())
}

fn next_map_part_one(map: &HashMap<(i64, i64), char>) -> Option<HashMap<(i64, i64), char>> {
    let mut something_changed = false;
    let next_map = map.iter()
        .map(|(key, value)|{ 
            let adjacent_occupied_seats_count = count_adjacent_occupied_seats_part_one(&map, *key);
            if *value == 'L' && adjacent_occupied_seats_count == 0 {
                something_changed = true;
                (*key, '#')
            } else if *value == '#' && adjacent_occupied_seats_count > 3 {
                something_changed = true;
                (*key, 'L')
            } else {
                (*key, *value)
            }
        })
        .collect();
    if something_changed {
        Some(next_map)
    } else {
        None
    }
}

fn count_adjacent_occupied_seats_part_one(map: &HashMap<(i64, i64), char>, (row, col): (i64, i64)) -> usize {
    [(1, 0),(1, 1),(1, -1),(-1, 0),(-1, 1),(-1, -1),(0, 1),(0, -1)]
        .iter()
        .map(|(row_mod, col_mod)| {
            map.get(&(row + row_mod, col + col_mod))
        })
        .filter(|value| value.is_some() && *value.unwrap() == '#')
        .count()
}

fn part_two(room: &Vec<Vec<char>>) -> Result<usize> {
    println!("init:");
    room.iter().for_each(|line| println!("{}", line.iter().collect::<String>()) );
    let mut map: HashMap<(i64, i64), char> = (0..room.len())
            .map(|row_index| (0..room[row_index].len()).map(move |col_index| ((row_index as i64, col_index as i64), room[row_index][col_index])))
            .flatten()
            .collect();


    let mut ic = 0;
    while let Some(next_map) = next_map_part_two(&map) {
        ic += 1;
        map = next_map;
        println!("Iteration count: {}", ic);
        (0..room.len())
            .for_each(|row_index| {
                (0..room[row_index].len()).for_each(|col_index| {
                    print!("{}", map.get(&(row_index as i64, col_index as i64)).unwrap())
                });
                println!("");
            });
        println!("\n");
    }

    Ok(map.iter().filter(|(_, v)| **v == '#').count())
}

fn next_map_part_two(map: &HashMap<(i64, i64), char>) -> Option<HashMap<(i64, i64), char>> {
    let mut something_changed = false;
    let next_map = map.iter()
        .map(|(key, value)|{ 
            let adjacent_occupied_seats_count = count_adjacent_occupied_seats_part_two(&map, *key);
            if *value == 'L' && adjacent_occupied_seats_count == 0 {
                something_changed = true;
                (*key, '#')
            } else if *value == '#' && adjacent_occupied_seats_count > 4 {
                something_changed = true;
                (*key, 'L')
            } else {
                (*key, *value)
            }
        })
        .collect();
    if something_changed {
        Some(next_map)
    } else {
        None
    }
}

fn count_adjacent_occupied_seats_part_two(map: &HashMap<(i64, i64), char>, (row, col): (i64, i64)) -> usize {
    [(1, 0),(1, 1),(1, -1),(-1, 0),(-1, 1),(-1, -1),(0, 1),(0, -1)]
        .iter()
        .map(|(row_mod, col_mod)| {
            (1..)
                .map(|index| map.get(&(row + index * row_mod, col + index * col_mod)))
                .take_while(std::option::Option::is_some)
                .flatten()
                .fold('.', |acc, v| {
                    if acc == '.' {
                        *v
                    } else {
                        acc
                    }
                })
        })
        .filter(|value| *value == '#')
        .count()
}

fn parse_input() -> Result<Vec<Vec<char>>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    Ok(input.lines().map(|s| s.chars().collect()).collect())
}
