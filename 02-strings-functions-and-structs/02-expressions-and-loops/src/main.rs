fn if_without_parens() {
    let year = 2022;

    if year > 2000 {
        println!("year is greater than 2000");
    } else {
        println!("year is not greater than 2000");
    }

    println!()
}

fn conditions_as_bools() {
    let xs: [i32; 0] = [];

    if xs.len() > 0 {
        println!("xs has length {}", xs.len());
    } else {
        println!("xs is empty");
    }

    if xs.is_empty() {
        println!("xs is empty with xs.is_empty()!");
    }

    println!();
}

fn variable_assignment_using_conditions() {
    let year = 1990;

    // variables can be assigned using multiple conditions; much more readable
    // than Javascript's nested ternaries
    let generation = if year > 1946 && year <= 1965 {
        "Boomer"
    } else if year > 1965 && year <= 1981 {
        "Gen x"
    } else {
        "younger"
    };

    println!("Generation for {} is '{}'", year, generation);

    println!();
}

fn loop_example() {
    let mut count: i32 = 0;

    loop {
        println!("looped: {count}");
        count += 1;

        if count >= 10 {
            break;
        }
    }

    println!();
}

fn variable_assignment_with_loop() {
    let mut count = 0;
    let x = loop {
        count += 1;

        if count >= 10 {
            break count * 2;
        }
    };

    println!("x: {x}");
    println!();
}

fn while_example() {
    let mut count = 0;

    while count <= 10 {
        println!("count: {count}");
        count += 1;
    }

    println!();
}

fn for_in_example() {
    let xs: [i8; 4] = [0, 1, 2, 3];

    for x in xs {
        println!("x is {x}");
    }

    println!();
}

fn pattern_matching_example() {
    #[derive(Debug)]
    enum Status {
        Connected,
        Disconnected,
        Unknown(String),
    }

    let mut connection = Status::Disconnected;

    match connection {
        Status::Connected => {
            connection = Status::Disconnected;
        }

        Status::Disconnected => {
            connection = Status::Connected;
        }

        // catchall
        _ => {
            connection = Status::Unknown(String::from("unknown - catch all happened"));
        }
    }

    println!("status: {connection:?}");
}

fn pattern_matching_if_let() {
    #[derive(Debug)]
    enum Foo {
        A,
        B,
        C,
    }
    let value = Foo::A;

    // this...
    if let Foo::A = value {
        println!("value is {value:?}");
    };

    // is syntatic sugar for this:
    match value {
        Foo::A => {
            println!("value is {value:?} - with catchall");
        }

        _ => {}
    }

    println!();
}

fn main() {
    // conditions
    if_without_parens();
    conditions_as_bools();
    variable_assignment_using_conditions();

    // loops
    loop_example();
    variable_assignment_with_loop();
    while_example();
    for_in_example();

    // pattern matching
    pattern_matching_example();
    pattern_matching_if_let();
}
