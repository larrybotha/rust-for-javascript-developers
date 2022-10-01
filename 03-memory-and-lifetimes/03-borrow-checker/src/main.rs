fn ownership_stack_variable_assignment() {
    let x = 1;
    let y = x;
    println!("{x}");
    println!("{y}");
    println!(
        "
        variables assigned to other variables containing values on the stack
        do not move ownership
        "
    );
}

fn ownership_heap_variable_assignment() {
    let x = String::from("foo");
    let y = x; // <= ownership moved to y because x is heap-allocated
               // x is no longer in scope

    println!("{y} - y is in scope \n");
}

fn ownership_function_calls() {
    fn inner(value: String) {
        println!("{}", value.trim());
    }

    let name = String::from("Joe Soap");
    inner(name);
    println!();
}

fn immutable_reference_function_call() {
    fn inner(value: &String) {
        println!("{value} - immutable reference in function",)
    }

    let x = String::from("foo");

    inner(&x);
    inner(&(x + " again"));
    println!();
}

fn immutable_reference_variable_assignment() {
    fn inner(value: &String) {
        println!("{value}: immutable ref");
    }

    let x = String::from("foo");
    let y = &x;

    inner(&x);
    inner(y);
    println!("x ({x}) and y ({y}) are safe to reference\n");
}

fn mutable_function_call() {
    fn inner(value: &mut String) {
        value.push_str(" and some");
    }

    let mut x = String::from("foo");

    inner(&mut x);
    println!("{x}\n");
}

fn mutable_variable_assigment() {
    let mut x = String::from("foo");
    let y = &mut x;

    y.push_str(" mutated via y"); // we can only mutate via y here
                                  // mutating x would be an attempt to
                                  // reference the value with another
                                  // reference, which is invalid while there
                                  // is a mutable reference

    println!("y ({y}) has the same value as x here\n");
}

fn main() {
    // ownership
    ownership_stack_variable_assignment();
    ownership_heap_variable_assignment();
    ownership_function_calls();

    // immutable references
    immutable_reference_function_call();
    immutable_reference_variable_assignment();

    // mutable references
    mutable_function_call();
    mutable_variable_assigment();
}
