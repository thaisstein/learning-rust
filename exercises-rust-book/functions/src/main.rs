fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}",five_plus_x ({
        let y = 7;
        y + 1
    }));
    print_labeled_measurements(5, 'h');
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is {}{}",value, unit_label);
}

fn five_plus_x(x: i32) -> i32 {
    5 + x
}