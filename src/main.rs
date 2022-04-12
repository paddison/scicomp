use std::io;

use std::env;
use gauss::{ helpers::*, Matrix };

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: '{0} -i' to enter rows,\n'{0} -f filename' to parse file", args[0]);
        std::process::exit(1);
    }

    let matrices = match args[1].as_str() {
        "-i" => from_user_input(),
        "-f" => {
            if args.len() == 3 {
                from_input_file(&args[2])
            } else {
                eprintln!("{} -f filename' to parse file", args[0]);
                std::process::exit(1); 
            }
        },
        invalid_arg => {
            eprintln!("Invalid argument: {0}\nUsage: '{1} -i' to enter rows,\n'{1} -f filename' to parse file", invalid_arg, args[0]);
            std::process::exit(1);
        }
    };
    
    let mut solutions = vec![];
    for (mut matrix, vector) in matrices {
        let solution = matrix.solve(vector);
        match solution {
            Some(solution) => solutions.push(solution),
            None => solutions.push(vec![f64::NAN]),
        }
    }

    // if solution is valid, ask if saved to file
    let file_name;
    if args.len() != 3 {
        file_name = "results";
    } else {
        file_name = &args[2]
    }
    write_result_to_file(file_name, solutions);
}

fn from_user_input() -> Vec<(Matrix, Vec<f64>)> {
    // Enter matrix
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut current_row = 1;
    let mut raw_matrix = vec![];
    println!("Enter matrix by rows. inputs must be comma separated and numeric.
Dimensions must be n by n.
It is allowed to write numbers like 1, 1.23 or 2/5.\n");

    // Get the first row separately to get dimemsion
    println!("Enter row {}:", current_row);
    stdin.read_line(&mut buffer).expect("Error reading input.");

    // SAFETY: Calling unwrap is safe, since the function always returns Ok when called with dimension == 0
    let dimension = parse_row_input(&mut raw_matrix, &buffer, 0).unwrap();
    // SAFETY: Calling unwrap is safe, since we check for the correct dimensions in the loop
    loop {
        let mut buffer = String::new();
        if current_row > dimension {
            break vec![(Matrix::from(&raw_matrix[0..(dimension * dimension)]),
                Vec::from(&raw_matrix[(dimension * dimension)..]))];
        }

        // handle user input
        if current_row != dimension {
            println!("Enter row {}:", current_row + 1);
        } else {
            println!("Enter vector to solve for");
        }
        stdin.read_line(&mut buffer).expect("Error reading input.");
        match parse_row_input(&mut raw_matrix, &buffer, dimension) {
                Ok(_) => { current_row += 1; }
                Err(_) => println!("Wrong dimension, must be {}", dimension),
        };
    }
}

#[test]
fn parse_row_input_split() {
    let mut raw_matrix: Vec<f64> = vec![];
    let buffer = String::from("2, 2.3, 3/5");
    let result = parse_row_input(&mut raw_matrix, &buffer, 3);
    assert_eq!(result, Ok(3));
    assert_eq!(raw_matrix, vec![2., 2.3, 0.6]);
}

