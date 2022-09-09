fn arrays() {
    println!("\nArrays\n");
    let xs = [1, 2, 3];

    // arrays do not implement Display, so we can't print them directly with
    // formatting macros, as we can scalars:
    //println!("{}", xs); // invalid

    // we can indicate to a formatting macro to use the Debug trait to render
    // arrays instead, using {:?}
    println!("{:?}", xs);

    // arrays may not contain mixed types
    //let invald_xs = ["foo", 2, 3];

    // mutable arrays can be initialised with a default value and length
    let mut ys: [&str; 3] = ["foo"; 3];
    ys[0] = "bar";
    ys[1] = "baz";
    ys[2] = "quux";

    println!("{:?}", ys);
    println!("length of ys: {}", ys.len());

    let long_list = [1, 2, 3, 4, 5];

    let [first, _, .., last] = long_list;

    println!("first: {first}");
    println!("last: {last}");
    println!();
}

fn tuples() {
    println!("Tuples\n");
    let x_tuple = ("foo", 1, true);

    println!("{x_tuple:?}");

    let mut y_tuple = ("bar", 2, false);

    // tuples may not be mutated in such a way that their types are mutated
    //y_tuple.0 = 1; // invalid - expected &str
    y_tuple.0 = "baz";

    println!("{y_tuple:?}");
}

fn main() {
    arrays();
    tuples();
}
