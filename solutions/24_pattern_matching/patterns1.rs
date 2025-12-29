fn main() {
    let number = 7;

    // Using match to categorize the number
    match number {
        0..=5 => println!("small"),
        6..=10 => println!("medium"),
        _ => println!("large"),
    }
}
