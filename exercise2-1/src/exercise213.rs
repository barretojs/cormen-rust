use std::io;

// Write an implementation for Linear Search

fn main() {
    let mut user_input = String::new();

    let mut array: Vec<i8> = vec![];

    let mut search: i8 = 0;

    let stdin = io::stdin();

    println!("Insert the comma separated array values:");

    stdin.read_line(&mut user_input);

    array = user_input
        .trim()
        .split(',')
        .collect::<Vec<_>>()
        .into_iter()
        .map(|number| number.trim().parse::<i8>().unwrap())
        .collect();

    user_input = String::new();

    println!("Insert the number to be searched in the array:");

    stdin.read_line(&mut user_input);

    search = user_input.trim().parse::<i8>().unwrap();

    for i in 0..array.len() - 1 {
        if array[i] == search {
            println!("Number index: {}", i as i8);
            return;
        }
    }

    println!("Number not found");
}
