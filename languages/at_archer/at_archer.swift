func main() {
    let line = readLine()!.split(separator: " ")
    let n: Int = Int(line[0])!
    let m: Int = Int(line[1])!
    let d: Int = Int(line[2])!
    let r = readLine()!.split(separator: " ").map { Int(String($0))! }
    let s = readLine()!.split(separator: " ").map { Int(String($0))! }
    let answer = solve(n: n, m: m, d: d, r: r, s: s)
    print(answer)
}

func solve(n: Int, m: Int, d: Int, r: [Int], s: [Int]) -> Int {
    // let => var
    var r = r
    var s = s
    // process
    let a = -(n * d / 2)
    let tmp_list = Array(r.reversed()).map { -$0 }.prefix(m) + r.map { $0 + 1 }.suffix(m)
    let l = tmp_list.prefix(2 * m - 1).map { $0 }
    let r_list = tmp_list.suffix(2 * m - 1).map { $0 }
    let s_list = Array(s.reversed()).map { $0 } + s.suffix(m - 1)
    var imos = [Int](repeating: 0, count: d + 1)
    for i in 0...(2 * m - 1) {
        if l[i] - a >= n * d {
            imos[0] -= n * s_list[i]
            imos[d] += n * s_list[i]
        } else if l[i] - a >= 1 {
            imos[0] -= (l[i] - a + d) / d * s_list[i]
            imos[(l[i] - a) % d] += s_list[i]
            imos[d] += (l[i] - a) / d * s_list[i]
        }
        if r_list[i] - a >= n * d {
            imos[0] += n * s_list[i]
            imos[d] -= n * s_list[i]
        } else if r_list[i] - a >= 1 {
            imos[0] += (r_list[i] - a + d) / d * s_list[i]
            imos[(r_list[i] - a) % d] -= s_list[i]
            imos[d] -= (r_list[i] - a) / d * s_list[i]
        }
    }
    var val = 0
    var score = 0
    for i in 0...d {
        val += imos[i]
        if val > score {
            score = val
        }
    }
    return score
}

main()
