use levin::levi_distance;

fn main() {
    let a = "hello";
    let b = "helo";
    println!("distance is :: {}", levi_distance(a, b));
    assert_eq!(1, levi_distance(a, b));
}
