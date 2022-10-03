use std::collections::HashMap;

fn simple_lifetimes() {
    let x = String::from("foo");

    {
        let y = String::from("bar");
        println!("y's lifetime ends at the end of this block: {y}");
    }

    println!("x's lifetime ends at the end of this function: {x}");
    println!();
}

fn lifetimes_with_references() {
    #[derive(Debug)]
    struct Thing<'a> {
        //       [1]
        thing_ref: &'a String,
        //         [2]
    }

    fn create_thing(value: &String) -> Thing {
        Thing { thing_ref: value }
    }

    let x = String::from("foo");
    let thing_a = create_thing(&x);
    let thing_b = create_thing(&x);
    //    [3]

    // 1 - we indicate that Thing contains a lifetime specified - there is
    //      some value in Thing that has a lifetime which cannot be inferred
    //      ny the compiler
    // 2 - the thing that has a lifetime that can't be inferred is thing_ref
    // 3 - the lifetimes of these Thing instances are no longer tied only to
    //      scope of their containing function - they are also tied to the
    //      lifetime of the scope of x - this is where the need for explicitly
    //      annotating lifetimes comes in

    println!("x: {x}");
    println!("thing_a.ref: {:?}", thing_a.thing_ref);
    println!("thing_b.ref: {:?}", thing_b.thing_ref);
    println!();
}

fn intro_to_option() {
    #[derive(Debug)]
    struct Person {
        name: Name,
    }
    #[derive(Debug)]
    struct Name {
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
    }

    let people = [
        Person {
            name: Name {
                first_name: String::from("Joe"),
                middle_name: Some(String::from("Monkey")),
                last_name: String::from("Soap"),
            },
        },
        Person {
            name: Name {
                first_name: String::from("Jane"),
                middle_name: None,
                last_name: String::from("Does"),
            },
        },
    ];

    println!("{people:?}\n");
}

fn multiple_lifetimes() {
    #[derive(Debug)]
    // both values inside this struct can use the same lifetime specifier as long
    // as the values are defined in the same scope when instantiating an instance
    struct Person<'a> {
        name: &'a str,
        location: &'a str,
    }

    fn get_city<'a>(code: &str, airport_codes: &'a HashMap<&str, &str>) -> &'a str {
        //      [1]                             ]2]                         [3]
        // 1 - we add a lifetime annotation to the function - the return value of
        //      this function has a lifetime defined outside of the scope of this
        //      function
        // 2 - the parameter where the returned value is obtained from is the
        //      hash map - the Rust compiler can now link the return value's
        //      lifetime with the value that it is derived from
        // 3 - the return value must have the same lifetime annotation - this is
        //      the value that has a lifetime which has been defined outside of
        //      this function

        // 'expects' is used hhere as a convenience - don't use it in production
        // unless you have good readon to end the running application process
        airport_codes.get(code).expect("Unknown airport code")
    }

    let airport_codes = HashMap::from([
        ("PIE", "St. Petersburg"),
        ("LHR", "London"),
        ("BOM", "Mumbai"),
    ]);
    // PIE exists, so Some is returned
    println!("PIE: {:?}", airport_codes.get("PIE"));
    // FOO does not exist, so None is returned
    println!("PIE: {:?}", airport_codes.get("FOO"));
    println!();

    let person = Person {
        name: "Joe Soap",
        location: "PIE",
    };

    println!("{:?}", person);
    println!(
        "Welcome {} from {}",
        person.name,
        get_city(person.location, &airport_codes)
    );
    println!();
}

fn static_lifetime() {
    static VERSION: u8 = 1;

    println!("version: {VERSION} - a static!");
}

fn main() {
    simple_lifetimes();
    lifetimes_with_references();

    // option primer
    intro_to_option();

    // multiple lifetimes
    multiple_lifetimes();

    static_lifetime();
}
