fn main() {
    let mut array: [i32; 6] = [5, 2, 4, 6, 1, 3];

    insertion_sort(&mut array);

    println!("Increased sorted array: {:?}", array);

    array = [5, 2, 4, 6, 1, 3];

    exercise212(&mut array);

    println!("Decreased sorted array: {:?}", array);
}

fn exercise212(array: &mut [i32]) -> () {
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
fn insertion_sort(array: &mut [i32]) -> () {
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
