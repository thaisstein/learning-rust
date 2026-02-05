fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    
    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    inferred_type = 3928492483i64;
    
    let mut mutable = 12;
    println!("mutable: {}", mutable);

    mutable = 21;
    println!("mutable after updating: {}", mutable);

    let mutable = true;
    println!("mutable after shadowing: {}", mutable);

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut count = 1;
    for element in my_array {
        println!("Element {}: {}", count, element);
        count += 1;
    }


    let my_tuple = (5u32, 1u8, true, -5.04f32);
    println!("First: {}", my_tuple.0);
    println!("Second: {}", my_tuple.1);
    println!("Third: {}", my_tuple.2);
    println!("Fourth: {}", my_tuple.3);

}