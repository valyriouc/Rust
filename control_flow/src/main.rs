fn main() {
    
    let number = 20;

    // If statements can be used to control the flow of the program 
    if number < 10 {
        println!("The number is lower than 10");
    } else if number < 20 {
        println!("The number is less than 20");
    } else {
        println!("The number is 20 or higher");
    }

    // If statements are expressions
    let val: u32 = if number < 15 { 20 } else if number == 15 { 25 } else { 30 };

    println!("Result: {val}"); 

    let mut counter = 0;

    // Used to run infinite
    let res = loop {
        
        if counter == 10 {
            break counter;
        }

        counter += 1;
    };

    println!("Result: {res}");

    // loop labels can be used
    let mut count = 0;
    'outer_loop: loop {
        let mut inner = 10;

        println!("Count: {count}");

        loop {
            if inner == 8 {
                break;
            }

            if count == 2 {
                break 'outer_loop;
            }

            println!("Inner: {inner}");
            inner -= 1;
        }

        count += 1;
    }

    let mut n = 3;

    // while for conditional loops
    while n != 0 {
        println!("Number: {n}");
        n -= 1;
    }

    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    for a in arr {
        println!("the value is: {a}");
    }

    for numb in 1..11 {
        print!("{numb} ");
    }
}   


