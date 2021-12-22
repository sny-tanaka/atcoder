use proconio::input;
 
fn main() {
    input! {
        n: u32,
        ab: [(usize, usize); n - 1],
    }
    let mut town = vec![Vec::new(); n as usize];
    for (a, b) in ab {
        town[a - 1].push(b - 1);
        town[b - 1].push(a - 1);
    }
    let dist = get_dist(n, &town, 0);
    let mut max: i32 = 0;
    let mut maxi: usize = 0;
    for (i, value) in dist.iter().enumerate() {
        if max < *value {
            max = *value;
            maxi = i;
        }
    }
    let dist = get_dist(n, &town, maxi);
    println!("{}", dist.iter().max().unwrap() + 1);
}
 
fn get_dist(n: u32, town: &Vec<Vec<usize>>, start: usize) -> Vec<i32> {
    let mut dist = vec![-1i32; n as usize];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    dist[start] = 0;
    while !queue.is_empty() {
        let pos: usize = queue.pop_front().unwrap();
        for next in &town[pos] {
            if dist[*next] == -1 {
                dist[*next] = dist[pos] + 1;
                queue.push_back(*next);
            }
        }
    }
    return dist;
}