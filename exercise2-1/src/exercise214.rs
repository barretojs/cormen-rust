use std::io;

// Write an implementation for binary addition of 2 numbers.

fn main() {
    let mut number_size = 0;

    let stdin = io::stdin();
    let mut user_input = String::new();

    println!("Enter the number size: ");

    stdin.read_line(&mut user_input);

    number_size = user_input.trim().parse::<usize>().unwrap();

    let mut num_1 = vec![0; number_size];
    let mut num_2 = vec![0; number_size];

    user_input = String::new();

    println!("Enter the first binary number, separated by comma: ");
    stdin.read_line(&mut user_input);

    num_1 = user_input
        .trim()
        .split(',')
        .collect::<Vec<_>>()
        .into_iter()
        .map(|number| number.trim().parse::<u8>().unwrap())
        .collect();

    user_input = String::new();

    println!("Enter the second binary number, separated by comma: ");
    stdin.read_line(&mut user_input);

    num_2 = user_input
        .trim()
        .split(',')
        .collect::<Vec<_>>()
        .into_iter()
        .map(|number| number.trim().parse::<u8>().unwrap())
        .collect();

    println!("First number: {:?}", num_1);
    println!("Second number: {:?}", num_2);

    binary_sum(&mut num_1, &mut num_2);
}

fn binary_sum(num_1: &Vec<u8>, num_2: &Vec<u8>) {
    let mut binary_1: Vec<_> = num_1.iter().rev().collect();
    let mut binary_2: Vec<_> = num_2.iter().rev().collect();

    let mut rev_binary = vec![];

    let mut carryover = 0;

    for i in 0..binary_1.len() {
        let sum = binary_1[i] + binary_2[i] + carryover;

        rev_binary.push(match sum {
            2 => 0,
            3 => 1,
            _ => sum,
        });

        if sum > 1 {
            carryover = 1;
        } else {
            carryover = 0;
        }
    }

    if carryover == 1 {
        rev_binary.push(1);
    }

    let result: Vec<_> = rev_binary.iter().rev().collect();

    println!("Result: {:?}", result);
}
