fn str_slice() {
    let str_slice: &str = "a slice!";
    let explicit_str_slice: &'static str = "a slice!";

    println!("str_slice: {str_slice}");
    println!("str_slice pointer: {:?}", str_slice.as_ptr());
    println!("str_slice length: {:?}", str_slice.len());
    println!("explicit_str_slice: {explicit_str_slice}");
    println!();
}

fn multi_line_str() {
    let x = "
        This is
        on
        multiple lines
    ";
    println!("{x}");
    println!();
}

fn str_escaping() {
    let x = "Escape \\this!";
    let y = r"Escape \without backslash using raw string!";

    println!("{x}");
    println!("{y}");
    println!();
}

fn string_operations() {
    let mut x = String::new();

    x.push_str("push_str mutates the original value");
    println!("{x}");

    let y = String::from("String::from can construct strings");
    println!("{y}");

    let mut z = String::from("Addition?");
    let a = String::from("ok?");
    z += &("Addition of strings requires all subsequent values to be string slices.".to_owned()
        + &a);
    println!("{z}");
    println!();
}

fn string_iterators() {
    let x = String::from("hello world");

    println!("some_string.chars returns an iterator:\n {:?}", x.chars());
    println!();

    let split_parts: Vec<&str> = x.split('e').collect();

    println!("some_string.split splits characters:\n {:?}", split_parts);
    println!();

    let white_space_parts: Vec<&str> = x.split_whitespace().collect();
    println!(
        "some_string.split_whitespace splits characters:\n {:?}",
        white_space_parts
    );
    println!();
}

fn main() {
    // &str
    str_slice();
    multi_line_str();
    str_escaping();

    // String
    string_operations();
    string_iterators();
}
