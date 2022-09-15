fn sum(a: u64, b: u64) -> u64 {
    // no need for 'return'
    a + b
}

// allow the enum to be debugged with {:?}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn eval_direction_enum() {
    let red = Direction::Up;
    let blue: Direction = Direction::Down;

    println!("up: {:?}", red);
    println!("down: {:?}", blue);
    println!("left: {:?}", Direction::Left);
    println!("right: {:?}", Direction::Right);
    println!();
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),  // rgb colors are always between 0 and 255, so we use u8
    Hsl(u16, u8, u8), // the first value is degrees in a circle, so we use u16
}

fn eval_color_enum() {
    let _red = Color::Red;
    let _blue = Color::Blue;
    let _green = Color::Green;
    let orange_rgb = Color::Rgb(251, 191, 36);
    let orange_hsl = Color::Hsl(43, 96, 56);

    println!("orange_rgb: {orange_rgb:?}");
    println!("orange_hsl: {orange_hsl:?}");
    println!();
}

fn main() {
    println!("sum of 1 and 2: {}", sum(1, 2));

    let anon_sum = |x, y| x + y;

    println!("anon sum of 1 and 2: {}", anon_sum(1, 2));
    // The following is invalid - we can't pass different types into the same
    // anonymous function
    //println!("anon sum of a and b: {}", anon_sum(1.0, 2.0));

    println!();
    eval_direction_enum();
    eval_color_enum()
}
