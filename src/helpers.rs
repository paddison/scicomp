use std::{fs::{File, OpenOptions}, io::{BufReader, BufRead, Write}};

use crate::Matrix;

pub fn from_input_file(file_name: &String) -> Vec<(Matrix, Vec<f64>)> {
    let mut results: Vec<(Matrix, Vec<f64>)> = vec![];
    let file = File::open(&file_name).expect(format!("Unable to find file: {}", file_name).as_str());
    let mut dimension = 0;
    let mut counter = 0;
    let mut raw_matrix= vec![];
    for line in BufReader::new(file).lines() {
        match line {
            Ok(line) => {
                if line.len() == 0 {
                    continue;
                }
                if counter == 0 {
                    // read in line, determine new dimension
                    if raw_matrix.len() != 0 {
                        results.push(
                            (Matrix::from(&raw_matrix[0..(dimension * dimension)]),
                            Vec::from(&raw_matrix[(dimension * dimension)..]))
                        )
                    }
                    raw_matrix = vec![];
                    match parse_row_input(&mut raw_matrix, &line, dimension) {
                        Ok(dim) => {
                            dimension = dim;
                            counter = dimension;
                        },
                        Err(e) => eprintln!("{}", e),
                    }
                } else {
                    // read in the rest of the lines and append to raw_matrix
                    parse_row_input(&mut raw_matrix, &line, dimension).expect("Error reading line");
                    counter -= 1;
                }
            },
            Err(e) => eprintln!("{}", e),
        };
        
    }
    results.push(
        (Matrix::from(&raw_matrix[0..(dimension * dimension)]),
        Vec::from(&raw_matrix[(dimension * dimension)..]))
    );
    results
}

pub fn parse_row_input(raw_matrix: &mut Vec<f64>, buffer: &str, dimension: usize) -> Result<usize, &'static str> {
    // todo split by commas, parse numbers by checking for '.' and '/' etc.
    // if dimension is 0, don't check for correct input length
    let numbers: Vec<&str> = buffer.split(',')
                                   .map(|n| n.trim())
                                   .filter(|n| !n.is_empty())
                                   .collect();

    let length = numbers.len();

    if dimension != 0 && length != dimension {
        return Err("Wrong dimensions");
    }

    for n in numbers {
        let n = match n.contains('/') {
            true => {
                n.split('/')
                 .map(|n| n.trim().parse::<f64>().expect("error parsing float"))
                 .reduce(|acc, n| acc / n )
                 .expect("Got '/' with no numbers.") 
            }
            false => n.parse::<f64>().expect("Error parsing number.")
        };
        raw_matrix.push(n);
    };
    Ok(length)
}

pub fn write_result_to_file(file_name: &str, solutions: Vec<Vec<f64>>) {
    let mut file = OpenOptions::new().append(true).open(&file_name).expect(format!("Unable to find file: {}", file_name).as_str());
    let mut buffer = String::from("\n");
    for vector in solutions {
        for n in vector {
            buffer += &(n.to_string() + ",");
        }
        buffer += "\n";
    }
    file.write_all(buffer.as_bytes()).expect("Error writing solutions to file.");
}