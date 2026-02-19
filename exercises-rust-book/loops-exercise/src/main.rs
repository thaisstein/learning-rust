fn main() {

    println!("{}", fibonacci_generator(6));
    xmas();
}

fn farenheit_celsius (temperature: f64) -> f64 {

    let temperature_celsius = (temperature - 32.0) * (5.0/9.0);
    temperature_celsius

}

fn fibonacci_generator (n: i32) -> i32 {

    if n == 0 { return 0; }
    if n == 1 { return 1; }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n { //to loop from 2 till n
        let fibonacci = a + b;
        a = b;
        b = fibonacci;
    };
    b    
}

fn xmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a-Laying",
        "seven Swans a-Swimming",
        "eight Maids a-Milking",
        "nine Ladies Dancing",
        "ten Lords a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];
    
    for i in 0..12 {
        println!{"on the {} day of xmas my true love sent to me:", days[i]};

        for j in (0..=i).rev() {
            println!("{}", gifts[j]);
        }
        println!("");
    }
}
