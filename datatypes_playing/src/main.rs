
fn main() {

    //// Scalar types 
    // Integer Types 
    let ivar8: i8 = 127;
    let uvar8: u8 = 127;

    let ivar16: i16 = -234;
    let uvar16: u16 = 3533;

    let ivar32: i32 = -23453;
    let uvar32: u32 = 324343;

    let ivar64: i64 = -2342343;
    let uvar64: u64 = 23423434;

    let ivar128: i128 = -23423943;
    let uvar128: u128 = 2342343234;

    let ivar_size: isize = -234234243;
    let uvar_size: usize = 234293298943;

    println!("{}", ivar8);
    println!("{}", uvar8);

    println!("{}", ivar16);
    println!("{}", uvar16);

    println!("{}", ivar32);
    println!("{}", uvar32);

    println!("{}", ivar64);
    println!("{}", uvar64);

    println!("{}", ivar128);
    println!("{}", uvar128);

    println!("{}", ivar_size);
    println!("{}", uvar_size);

    // Floating point types
    let fvar32: f32 = 43.3;
    let fvar64: f64 = 23423.123; // Default type

    println!("{}", fvar32);
    println!("{}", fvar64);

    // Operator: https://doc.rust-lang.org/book/appendix-02-operators.html

    // Boolean types
    let bvar: bool = true;

    println!("{}", bvar);

    // Character types
    // Char type has four bytes (represents a unicode scalar value) 
    let c = 'z';
    let z: char = 'Z';

    println!("{}", c);
    println!("{}", z);

    //// Compound Types - group multiple values into one type
    
    // tuples
    // Grouping number of values with variety of types 
    // have fixed length
    let tup = (230, 43.3, true);

    println!("{0} : {1} : {2}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;

    println!("{}", y);

    // array
    // Every value must have the same value
    // have fixed length 
    // stack allocated
    let a: [u8; 5]= [1,2,3,4,5];

    println!("{}", a[0]);

    let b = [3; 5];

    println!("{}", b[2]);
     
    println!("{}", b[10]);
}
