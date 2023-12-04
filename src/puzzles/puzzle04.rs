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

    }
}