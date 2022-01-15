use proconio::input;
 
struct Answer {
    a: Vec<usize>,
    b: Vec<usize>,
    is_impossible: bool,
}
impl Answer {
    fn new() -> Answer {
        return Answer {
            a: Vec::new(),
            b: Vec::new(),
            is_impossible: false,
        };
    }
}
 
fn solve(m: usize, a: Vec<usize>) -> Answer {
    let mut ans = Answer::new();
    if m == 1 {
        ans.a.push(a[0]);
        if a[0] == 1 {
            ans.b.push(a[0]);
        }else {
            ans.b.push(a[0] - 1);
            ans.b.push(1usize);
        }
    }else {
        let mut odd = Vec::new();
        for an in a {
            if an % 2 == 0 {
                ans.a.push(an);
            }else {
                odd.push(an);
            }
        }
        let odd_cnt = odd.len();
        if odd_cnt > 2 {
            ans.is_impossible = true;
        }else {
            if odd_cnt > 0 {
                ans.a.insert(0, odd[0]);
            }
            if odd_cnt > 1 {
                ans.a.push(odd[1]);
            }
            ans.b = ans.a.clone();
            ans.b[m - 1] += 1;
            if ans.b[0] > 1 {
                ans.b[0] -= 1;
            }else {
                ans.b.remove(0);
            }
        }
    }
    return ans;
}
 
fn main() {
    input! {
        _n: usize,
        m: usize,
        a: [usize; m],
    }
    let ans = solve(m, a);
    if ans.is_impossible {
        println!("Impossible");
    }else {
        println!("{}", ans.a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        println!("{}", ans.b.len());
        println!("{}", ans.b.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}