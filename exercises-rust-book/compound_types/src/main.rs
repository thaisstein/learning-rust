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
}
