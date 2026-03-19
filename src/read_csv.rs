// Reads the data from the CSV file and returns a vector of tuples containing the address and amount.
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_csv(file_path: &str) -> Result<Vec<(String, u64)>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines().skip(1) {
        // Skip the header
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let address = parts[0].to_string();
            let amount: u64 = parts[1].parse()?;

            data.push((address, amount));
        }
    }

    Ok(data)
}
