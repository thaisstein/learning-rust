fn main() {
    let mut counter = 0;


    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        counter += 1;
    }
    println!("End count = {counter}");
}
