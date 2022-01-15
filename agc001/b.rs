use proconio::input;
 
fn solve(n: usize, x: usize) -> usize {
    return 3 * (n - gcd(n, x));
}
 
fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        return b;
    }else {
        return gcd(b, a % b);
    }
}
 
fn main() {
    input! {
        n: usize,
        x: usize,
    }
    println!("{}", solve(n, x));
}