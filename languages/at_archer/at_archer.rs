use proconio::input;
 
fn main() {
    input! {
        n: i64,
        m: i64,
        d: i64,
        r: [i64; m as usize + 1],
        s: [i64; m as usize],
    }
    let answer = solve(n, m, d, r, s);
    println!("{}", answer);
}

fn solve(n: i64, m: i64, d: i64, r: Vec<i64>, mut s: Vec<i64>) -> i64 {
    let a = -(n * d / 2);
    let mut tmp_vec: Vec<i64> = r.iter().rev().map(|x| -x).collect::<Vec<i64>>();
    tmp_vec = tmp_vec[..m as usize].to_vec();
    tmp_vec.append(&mut r.iter().map(|x| x + 1).collect::<Vec<i64>>()[1..].to_vec());
    let l = tmp_vec[..tmp_vec.len() - 1].to_vec();
    let r_vec = tmp_vec[1..].to_vec();
    let mut s_vec: Vec<i64> = Vec::new();
    for s0 in s.iter().rev() {
        s_vec.push(*s0);
    }
    s_vec.append(&mut s[1..].to_vec());
    let mut imos = vec![0; d as usize + 1];
    for i in 0..(2 * m as usize - 1) {
        if l[i] - a >= n * d {
            imos[0] -= n * s_vec[i];
            imos[d as usize] += n * s_vec[i];
        } else if l[i] - a >= 1 {
            imos[0] -= (l[i] - a + d) / d * s_vec[i];
            imos[((l[i] - a) % d) as usize] += s_vec[i];
            imos[d as usize] += (l[i] - a) / d * s_vec[i];
        }
        if r_vec[i] - a >= n * d {
            imos[0] += n * s_vec[i];
            imos[d as usize] -= n * s_vec[i];
        } else if r_vec[i] - a >= 1 {
            imos[0] += (r_vec[i] - a + d) / d * s_vec[i];
            imos[((r_vec[i] - a) % d) as usize] -= s_vec[i];
            imos[d as usize] -= (r_vec[i] - a) / d * s_vec[i];
        }
    }
    let mut val: i64 = 0;
    let mut score: i64 = 0;
    for i in 0..d as usize {
        val += imos[i];
        score = std::cmp::max(score, val);
    }
    return score;
}