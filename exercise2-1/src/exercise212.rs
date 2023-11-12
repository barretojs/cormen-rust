use std::io;

fn main() {
    let stdin = io::stdin();

    let mut user_input = String::new();

    let mut array: Vec<i8> = vec![];

    println!("Insert the comma separated array values:");

    stdin.read_line(&mut user_input);

    array = user_input
        .trim()
        .split(',')
        .collect::<Vec<_>>()
        .into_iter()
        .map(|number| number.trim().parse::<i8>().unwrap())
        .collect();

    insertion_sort(&mut array);

    println!("Increased sorted array: {:?}", array);

    exercise212(&mut array);

    println!("Decreased sorted array: {:?}", array);
}

fn exercise212(array: &mut Vec<i8>) -> () {
    //Rewrite the insertion-sort procedure to sort into non increasing instead of non-decreasing order
    //Just check for array(i) < key

    for j in 1..array.len() {
        let key = array[j];

        let mut i = j as i64 - 1;

        while i >= 0 && array[i as usize] < key {
            array[(i + 1) as usize] = array[i as usize];
            i -= 1;
        }

        array[(i + 1) as usize] = key;
    }
}

// Passes the array slice so we change the values by reference
fn insertion_sort(array: &mut Vec<i8>) -> () {
    for j in 1..array.len() {
        let key = array[j];

        /*
            the cast has to be made because rust uses USIZE as the array index, which only goes to 0. To follow the algorithm just as proposed by Cormen this is the only way it works. Probably a better option would be to use the .swap() function on the array isntead of doing manually.
        */

        let mut i: i64 = j as i64 - 1;

        while i >= 0 && array[i as usize] > key {
            array[(i + 1) as usize] = array[i as usize];
            i -= 1;
        }

        array[(i + 1) as usize] = key;
    }
}
