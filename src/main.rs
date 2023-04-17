use std::io::{self, Write};

fn sum_strings_from_input() -> bool {
    let mut total_secs = 0;

    loop {
        print!("Enter a time string in the format 0:34 (or press Enter to finish): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim any leading/trailing whitespace
        let input = input.trim();

        if input.is_empty() {
            // Exit loop when input is empty
            break;
        }

        // Parse input into minutes and seconds
        let parts: Vec<&str> = input.split(':').collect();

        if parts.len() != 2 {
            println!("Invalid input. Please enter a time string in the format 0:34.");
            continue;
        }

        let minutes: u64 = parts[0].trim().parse().unwrap_or(0);
        let seconds: u64 = parts[1].trim().parse().unwrap_or(0);

        // Add time to total seconds
        total_secs += (minutes * 60) + seconds;
    }

    if total_secs > 0 {
        let num = {Some(total_secs).unwrap()};
        println!("{:?} seconds", num);
    }
    false
}

fn main() {
    sum_strings_from_input();
}