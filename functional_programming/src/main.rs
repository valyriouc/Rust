fn main() {
    let iterator = [1,2,3,4,5,6,7,8].into_iter();

    for i in iterator.clone() {
        println!("{i}");
    }

    let iter: Vec<i32> = iterator.skip(2).collect();
    iter.iter().for_each(|x| print!(" {x}"));

    println!();

    let result: Vec<char> = (0..10).into_iter().map(|x| '#').collect();
    result.into_iter().for_each(|x| print!("{x}"));

    println!();

    let number = std::iter::repeat(String::from("+")).take(5).reduce(|acc, x| acc + &x);

    let content = match number {
        Some(num) => "Hello",
        None => "Goodbye"
    };

    print!("{content}");
}
