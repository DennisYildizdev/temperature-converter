use std::io;

fn main() {
    let mut number1 = String::new();
    let mut temperature_function = String::new();
    let conversion_factor: f32 = 1.8;
    let offset: f32 = 32.0;

    println!("please input temperature");
    io::stdin()
        .read_line(&mut number1)
        .expect("error");

    println!("please input (°C or °F)");
    io::stdin()
        .read_line(&mut temperature_function)
        .expect("error");

    let number1: f32 = number1.trim().parse().expect("error");

    let result = match temperature_function.trim() {
        "c" => number1 * conversion_factor + offset,
        // Fahrenheit to Celsius: (°F - 32.0) / 1.8
        "f" => (number1 - offset) / conversion_factor,
        _ => {
            println!("error");
            return;
        }
    };
    println!("result: {}", result);
}