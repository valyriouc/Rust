
fn main() {
 
    let test = fib_while(14);
    println!("{test}");
}

fn fib_while(n: i32) -> i32 {
    let mut count = 0;
    let mut sum = 0;
    let mut last = 0;

    while count < n - 1{
        if count == 0 {
            last = 1;
        } else {
            last = sum - last;
        }

        sum += last;
        count += 1;
    }

    sum
}