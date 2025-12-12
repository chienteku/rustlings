#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    // Option 1:
    match optional_point {
        Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    // Option 2:
    // match &optional_point {
    //     Some(p) => println!("Coordinates are {}, {}", p.x, p.y),
    //     _ =>panic!("No match!"),
    // }

    println!("{optional_point:?}"); // Don't change this line.
}
