fn i32_stack() {
    let my_int: i32 = 123;

    println!("size of my_int ({my_int}): i32 is always 4 butes");
    println!("my_int is thus stack-allocated");
    println!();
}

fn string_heap() {
    let mut my_string = String::from("foo");

    my_string = my_string + " " + "bar";

    println!("{my_string}");
    println!("my_string is growable, and thus not fixed, and thus is heap allocated");
    println!();
}

fn force_heap() {
    let my_int = Box::new(123);

    println!("{my_int}");
    println!("my_int is forcefully allocated to the heap using `Box`");
    println!();
}

fn main() {
    i32_stack();
    string_heap();
    force_heap();
}
