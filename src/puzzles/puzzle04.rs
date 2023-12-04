use crate::get_lines;

pub fn main()
{
    let file_path = "./04/example_input.txt";

    let lines_vector = get_lines(file_path);

    // Iterate over lines
    for line in lines_vector
    {
        let mut card_points = 0u32;

        // Split line on : and |
        let mut line_split = line.split(&[':', '|'][..]);

        // Get winning numbers Vector
        let mut winning_nums: Vec<&str> = line_split
            .next()
            .expect("Unable to split winning numbers.")
            .trim()
            .split(" ")
            .collect();

        // Get card numbers Vector
        let mut card_numbers: Vec<&str> = line_split
            .next()
            .expect("Unable to split numbers.")
            .trim()
            .split(" ")
            .collect();
        // Remove blank string items from splitting
        winning_nums.retain(|&n| n != "");
        card_numbers.retain(|&n| n != "");

        println!("Winning Numbers:");
        for win_num in winning_nums
        {
            println!("{}", win_num);
        }
        println!("Card Numbers:");
        for card_num in card_numbers
        {
            println!("{}", card_num);
        }
    }
}