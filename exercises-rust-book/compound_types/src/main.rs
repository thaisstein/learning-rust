use std::io;

fn main() {
    // tuple
    let tup = (500, 6.4, 1);
    let(x, y, z) = tup;
    println!("{}", tup.2);

    // array
    let a = [1, 2, 3, 4, 5];
    let months = ["jan", "fev", "march", "apr", "may", "june", "july", "aug", "sept", "oct", "nov", "dec"];
    let b: [i32; 5] = [1, 2, 3, 4, 5] ;// type, # of elements
    let c = [3;5]; //his creates an array [3, 3, 3, 3, 3]
    println!("element {}", months[4]);

    let mut index = String::new(); //for input

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index: usize = index //transforms in inter
    .trim()
    .parse()
    .expect("Index entered was not a number");

    let element = months[index];

    println!("Month {} is: {}", index + 1, element);
}
