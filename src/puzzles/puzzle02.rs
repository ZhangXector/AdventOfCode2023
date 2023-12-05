use std::collections::HashMap;
use std::fs;

pub fn main()
{
    let file_path = "./02/input.txt";

    let cubes = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let lines_vector: Vec<String> = fs::read_to_string(file_path)
        .expect("Problem opening the file: {file_path}")
        .lines()
        .map(String::from)
        .collect();

    let mut total = 0u32;
    // TODO: get game number from the string split, if you have time
    let mut game_num = 1u32;
    for line in lines_vector
    {
        let separate = line.split(": ").collect::<Vec<&str>>();

        for sets in separate[1].split("; ").collect::<Vec<&str>>()
        {
            for draw in sets.split(", ").collect::<Vec<&str>>()
            {
                let split_draw = draw.split(" ").collect::<Vec<&str>>();
                let number: u32 = split_draw[0].parse().expect("Unable to parse number.");
                let color = split_draw[1];
                let max = cubes[color];
                if (number > max)
                {
                    total += game_num;
                }
            }
        }
        game_num += 1;
    }
    println!("{total}");
}