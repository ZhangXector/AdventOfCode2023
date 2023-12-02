use std::fs;
use std::collections::HashMap;

pub fn main()
{
    let file_path = "./01/input.txt";
    let string_nums = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);

    let lines_vector: Vec<String> = fs::read_to_string(file_path)
        .expect("Problem opening the file: {file_path}")
        .lines()
        .map(String::from)
        .collect();

    let mut total: u32 = 0;
    for line in lines_vector
    {
        /*
        let mut fixed_string = String::from(line);
        for (str_num, value) in &string_nums
        {
            fixed_string = fixed_string.replace(str_num, value);
        }
        */
        let v: Vec<&str> = line.matches(|c: char| c.is_ascii_digit()).collect();
        let first = v.first().expect("Unable to fetch first.");
        let last = v.last().expect("Unable to fetch last.");
        let mut double_char: String = String::from("");
        double_char.push_str(first);
        double_char.push_str(last);
        let double_digit: u32 = double_char.parse().expect("Unable to parse string to integer.");
        total += double_digit;
    }
    println!("Total: {}", total);
}