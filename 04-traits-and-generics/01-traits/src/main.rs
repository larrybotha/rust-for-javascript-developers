use std::fmt;

fn custom_debug() {
    struct Thing {
        x: i32,
        y: i32,
    }

    impl fmt::Debug for Thing {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Thing")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }

    let x = Thing { x: 10, y: 20 };

    println!("{x:#?}\n");
}

fn custom_trait() {
    trait LoudNoises {
        fn shout(&self) -> String;
    }

    struct Person {
        name: String,
    }

    impl LoudNoises for Person {
        fn shout(&self) -> String {
            let message = format!("my name is {}!!!", &self.name).to_uppercase();

            message
        }
    }

    struct Alien {
        flurber: String,
    }

    impl LoudNoises for Alien {
        fn shout(&self) -> String {
            let message = format!("qrbi lburd !!{}!! hurbzgr", &self.flurber);

            message
        }
    }

    // shout has a type constraint that the value passed in must implement
    // the LoudNoises trait
    fn shout<T: LoudNoises>(shouter: T) -> String {
        shouter.shout()
    }

    let joe = Person {
        name: "Joe Soap".to_string(),
    };
    let bgrilb = Alien {
        flurber: "bgrilb frb".to_string(),
    };

    println!("peoples shout like: {:?}", shout(joe));
    println!("aliens shout like: {:?}", shout(bgrilb));
    println!();
}

fn main() {
    custom_debug();
    custom_trait();
}
