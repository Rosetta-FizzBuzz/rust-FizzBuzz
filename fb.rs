fn fizz_buzz(n: i32) {
    let mut i = 1;
    let f = "Fizz";
    let b = "Buzz";
    while i <= n {
        if i % 15 == 0 {
            println!("{}{}", f, b);
        } else if i % 5 == 0 {
            println!("{}", b);
        } else if i % 3 == 0 {
            println!("{}", f);
        } else {
            println!("{}", i);
        }
        i += 1;
    }
}
 
fn main() {
    fizz_buzz(100);
}
