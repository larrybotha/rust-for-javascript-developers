fn main() {
    let integer: u64 = 5;
    let decimal: f64 = 1.5;
    let character: char = 'S';
    let boolean: bool = true;

    println!("integer: {integer}");
    println!("decimal: {decimal}");
    println!("character: {character}");
    println!("boolean: {boolean}");

    // math operators do not work on integers and floats unless they are
    // explicitly cast to the matching type
    let a = 5;
    let b = 5.0;
    let sum = f64::from(a) + b;
    println!("sum: {sum}");
}
