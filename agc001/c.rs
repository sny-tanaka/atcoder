use proconio::input;
 
struct Param {
    n: usize,
    k: usize,
    tree: Vec<Vec<usize>>,
    depth: Vec<usize>,
}
impl Param {
    fn new(n: usize, k: usize, tree: Vec<Vec<usize>>) -> Param {
        return Param {
            n,
            k,
            tree,
            depth: vec![0usize; n],
        };
    }
    fn main(&mut self) -> usize {
        let mut res = self.n;
        if self.k % 2 == 0 {
            for v in 0..self.n {
                self.rec(v, -1, 0);
                let mut num = 0usize;
                for i in 0..self.n {
                    if self.depth[i] > self.k / 2 {
                        num += 1;
                    }
                }
                res = std::cmp::min(res, num);
            }
        }else {
            for u in 0..self.n {
                for i in 0..self.tree[u].len() {
                    let v = self.tree[u][i];
                    self.depth[v] = 0;
                    self.rec(u, v as isize, 0);
                    self.rec(v, u as isize, 0);
                    let mut num = 0usize;
                    for j in 0..self.n {
                        if self.depth[j] > self.k / 2 {
                            num += 1;
                        }
                    }
                    res = std::cmp::min(res, num);
                }
            }
        }
        return res;
    }
    fn rec(&mut self, v: usize, p: isize, curdepth: usize) {
        self.depth[v] = curdepth;
        for i in 0..self.tree[v].len() {
            let nv = self.tree[v][i];
            if nv as isize != p {
                self.rec(nv, v as isize, curdepth + 1);
            }
        }
    }
}
fn solve(n: usize, k: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut tree = vec![Vec::new(); n];
    for (a, b) in ab {
        tree[a - 1].push(b - 1);
        tree[b - 1].push(a - 1);
    }
    let mut p = Param::new(n, k, tree);
 
    return p.main();
}
 
fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n - 1],
    }
    println!("{}", solve(n, k, ab));
}