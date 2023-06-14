fn main() {
    
    let v1 = 40;
    let v2 = v1;

    println!("{v1}");
    println!("{v2}");

    let s1 = String::from("hello");

    hello(s1);

    let mut s2 = return_from_function();

    println!("{s2}");

    s2 = in_and_out(s2);

    println!("{s2}");

}

fn hello(s1: String) -> () {
    println!("{s1}");
}

fn return_from_function() -> String {

    String::from("world")
}

fn in_and_out(s1: String) -> String {
    println!("{s1}");

    s1
}