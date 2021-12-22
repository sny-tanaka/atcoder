use proconio::input;
 
struct Param {
    n: u32,
    l: u32,
    k: u32,
    a: Vec<u32>,
}
impl Param {
    fn solve(&mut self, mid: &u32) -> bool {
        let mut cnt: u32 = 0;
        let mut pre: u32 = 0;
        for i in 0..self.n as usize {
            if self.a[i] - pre >= *mid && self.l - self.a[i] >= *mid {
                cnt += 1;
                pre = self.a[i];
            }
        }
        return cnt >= self.k;
    }
    fn main(&mut self) {
        let mut left: u32 = 0;
        let mut right: u32 = self.l;
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if self.solve(&mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        println!("{}", left);
    }
}
 
fn main() {
    input! {
        n: u32,
        l: u32,
        k: u32,
        a: [u32; n],
    }
    let mut p = Param {
        n,
        l,
        k,
        a,
    };
    p.main();
}