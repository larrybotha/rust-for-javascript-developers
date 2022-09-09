fn main() {
    let immutable_name = "John Doe";

    println!("immutable_name: {}", immutable_name);

    // invalid - immutable_name is immutable
    //immutable_name = "Jane Smith"

    let assign_later;
    assign_later = "Jane Doe";
    println!("assign_later: {}", assign_later);

    let mut mutable_name = "John Doe";
    mutable_name = "Jane Doe";

    println!("mutable_name: {}", mutable_name);

    const IS_CONSTANT: bool = true;

    println!("is_constant: {}", IS_CONSTANT)
}
