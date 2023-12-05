use std::fs;
pub mod puzzles;

use crate::puzzles::puzzle01;
use crate::puzzles::puzzle04;
use crate::puzzles::puzzle02;

fn main()
{
    //puzzle01::main();
    //puzzle02::main();
    puzzle04::main();
}

/// Opens a file path and reads into a Vector<String> all lines for parsing
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path of the file to open
///
/// # Returns
///
/// * Vec<string> with all lines from the file
fn get_lines(file_path: &str) -> Vec<String>
{
    // Fetch lines from file as Vector of String objects
    let lines_vector: Vec<String> = fs::read_to_string(file_path)
        .expect("Problem opening the file: {file_path}")
        .lines()
        .map(String::from)
        .collect();

    return lines_vector;
}