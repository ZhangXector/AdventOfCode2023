use std::fs;

pub fn main()
{
    let file_path = "./04/example_input.txt";

    let lines_vector: Vec<String> = fs::read_to_string(file_path)
        .expect("Problem opening the file: {file_path}")
        .lines()
        .map(String::from)
        .collect();

    for line in lines_vector
    {
        let mut line_split = line.split(&[':', '|'][..]);
        let card = line_split.next().expect("Unable to split card numbers.").trim();
        let winning_nums = line_split.next().expect("Unable to split winning numbers.").trim();
        let numbers = line_split.next().expect("Unable to split numbers.").trim();
        println!("{}\n{}\n{}", card, winning_nums, numbers);
    }
}