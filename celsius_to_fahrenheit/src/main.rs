
fn main() {
    let val = convert_to_fahrenheit(-1.0);
    println!("{val}");

    let var = convert_to_celsius(71.6);
    println!("{var}");
}

fn convert_to_fahrenheit(deg: f64) -> f64 {
    deg * (9.0/5.0) + 32.0 
}

fn convert_to_celsius(fahr: f64) -> f64 {
    (fahr - 32.0) / 1.8
}