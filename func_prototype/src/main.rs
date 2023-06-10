fn main() {
    println!("Hello, world!");
    another_print();
    print_value(200);
    let value = calculate();

    println!("{value}");
}

fn another_print() {
    println!("Nice to meet you");
}

fn print_value(value: i32) {
    println!("The value is {value}");
} 


fn calculate() -> i32{
    let x = 4 + 34;
    x
}

// Statement = perform some action but do not return a value

// Expression = evaluate to a result value
