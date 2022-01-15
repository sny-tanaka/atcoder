use proconio::input;
 
fn solve(n: u32, mut l: Vec<u32>) -> u32 {
    let mut answer: u32 = 0;
    l.sort();
    for i in 0..n as usize {
        answer += l[i * 2];
    }
    return answer;
}
 
fn main() {
    input! {
        n: u32,
        l: [u32; n * 2],
    }
    println!("{}", solve(n, l));
}