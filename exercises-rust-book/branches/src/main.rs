use std::io;

fn main() {
    let mut number = String::new();

    io::stdin()
    .read_line(& mut number)
    .expect("Error reading line");

    let number: u32 = number.trim().parse().expect("Enter a valid number");

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }  if number % 3 == 0 {
        println!("number is divisible by 3")
    }  if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 4 };
    println!("The value of number is: {number}");

    let x;
    let cond = false;
    if cond {
    x = 1;
    } else {
    x = 2;
    }
    println!("{}", x);
}
