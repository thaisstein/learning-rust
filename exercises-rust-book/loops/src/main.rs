fn main() {
    let a = [10, 20, 30, 40, 50];
    let b = [5;10];
    let mut sum = 0;
    for element in a {
        println!("the value is: {element}");
    } 
    for x in b {
        sum +=x;
    }
    println!("{}", sum);
    //rev
    for number in (1..5).rev() {
        println!("{}!",number);
    }  
    println!("liftoff!");
}
