fn fizz_buzz(n: int) {
    let mut i = 1;
    let f = "Fizz";
    let b = "Buzz";
    while i <= n {
        if i % 15 == 0 {
            println!("{:s}", f + b);
        } else if i % 5 == 0 {
            println!("{:s}", b);
        } else if i % 3 == 0 {
            println!("{:s}", f);
        } else {
            println!("{:i}", i);
        }
        i += 1;
    }
}
 
fn main() {
    fizz_buzz(100);
}
