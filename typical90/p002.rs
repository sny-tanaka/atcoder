use proconio::input;
 
fn main() {
    input! {
        n: u8,
    }
    if n % 2 == 0 {
        for bit in 0..(1 << n) {
            let mut cnt: i8 = 0;
            let mut ans: String = String::from("");
            let mut is_correct: bool = true;
            for i in (0..n).rev() {
                if bit & (1 << i) == 0 {
                    cnt += 1;
                    ans.push('(');
                } else {
                    cnt -= 1;
                    ans.push(')');
                }
                if cnt < 0 {
                    is_correct = false;
                    break;
                }
            }
            if is_correct && cnt == 0 {
                println!("{}", ans);
            }
        }
    }
}