
fn parity(x: i64) -> &'static str {
    if x % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}
fn add(a: i64, b: i64) -> i64 {
    return a + b; // Change this line
}

fn main() {
    let a = 5;
    let b = 6;
    println!("Parity of {} + {} is {}", a, b, parity(add(a, b)));
} 
