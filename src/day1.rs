use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    // Define the input file path
    let file_path = "1_input.txt";

    // Initialize vectors to store the two columns
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    // Open the file and read lines
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(record) = line {
                // Split each line into parts and parse as integers
                let parts: Vec<&str> = record.split_whitespace().collect();
                if parts.len() >= 2 {
                    match (parts[0].parse(), parts[1].parse()) {
                        (Ok(num1), Ok(num2)) => {
                            col1.push(num1);
                            col2.push(num2);
                        }
                        _ => eprintln!("Failed to parse line: {}", record),
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to open file: {}", file_path);
        return;
    }

    // Sort the columns independently
    col1.sort_unstable();
    col2.sort_unstable();

    // Calculate the sum of absolute differences
    let total_distance: i32 = col1.iter()
        .zip(col2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Total Distance: {}", total_distance);

    // Part 2: Calculate the running total
    // Create a frequency map for col2
    let mut frequency_map: HashMap<i32, usize> = HashMap::new();
    for &num in &col2 {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    // Compute the running total for part 2
    let mut running_total = 0;
    for &num in &col1 {
        if let Some(&count) = frequency_map.get(&num) {
            running_total += num * count as i32;
        }
    }

    println!("Part 2 Running Total: {}", running_total);
}

// Utility function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}