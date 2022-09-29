#[derive(Debug)]
struct Person {
    age: u8,
    name: String,
}

#[derive(Debug)]
struct RGBColor(u8, u8, u8);

fn struct_intro() {
    let person = Person {
        age: 80,
        name: String::from("Joe Soap"),
    };

    println!("person: {:?}", person);
    println!("age: {:?}", person.age);
    println!("name: {:?}", person.name);
    println!();
}

fn struct_property_shorthands() {
    let age = 30;
    let name = String::from("Jane Doe");
    let person = Person { age, name };

    println!("defined using property shorthand: {:?}", person);
    println!();
}

fn structs_with_spread() {
    let person_a = Person {
        age: 20,
        name: String::from("Jack Sprat"),
    };
    let person_b = Person {
        name: String::from("Not ") + &person_a.name,
        // spread person_a onto person_b
        ..person_a
    };

    println!("person_a, {:?}", person_a);
    println!("person_b, {:?}", person_b);
    println!();
}

fn tuple_struct_intro() {
    let rgb = RGBColor(10, 20, 30);

    println!("rgb: {:?}", rgb);
    println!();
}

fn tuple_accessors() {
    let rgb = RGBColor(100, 200, 50);

    println!("rgb.0: {:?}", rgb.0);
    println!("rgb.1: {:?}", rgb.1);
    println!("rgb.2: {:?}", rgb.2);
    println!();
}

fn impl_example_method() {
    struct Foo {
        foo: String,
    }

    impl Foo {
        fn join(&self, value: String) -> String {
            let words = format!("{} {}", self.foo, value);

            words
        }
    }

    let thing = Foo {
        foo: String::from("foo"),
    };
    let words = thing.join(String::from("bar!"));

    println!("words: {words}");
    println!();
}

fn impl_example_associated_function() {
    #[derive(Debug)]
    struct Foo {
        foo: String,
    }

    impl Foo {
        fn from(value: &str) -> Foo {
            Foo {
                foo: String::from(value),
            }
        }
    }

    let thing = Foo::from("bar");

    println!("thing from associated method: {thing:?}");
    println!("foo: {:?}", thing.foo);
    println!();
}

fn main() {
    // structs
    struct_intro();
    struct_property_shorthands();
    structs_with_spread();

    // tuple structs
    tuple_struct_intro();
    tuple_accessors();

    // methods
    impl_example_method();
    impl_example_associated_function();
}
