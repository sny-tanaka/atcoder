def main():
    n, m, d = map(int, input().split())
    r = list(map(int, input().split()))
    s = list(map(int, input().split()))
    answer = solve(n, m, d, r, s)
    print(answer)

def solve(n: int, m: int, d: int, r: list, s: list) -> int:
    a = -(n * d // 2)
    tmp_list = [-x for x in reversed(r)][:-1] + [x + 1 for x in r][1:]
    l = tmp_list[:-1]
    r_list = tmp_list[1:]
    s_list = list(reversed(s)) + s[1:]
    imos = [0] * (d + 1)
    for i in range(2 * m - 1):
        if l[i] - a >= n * d:
            imos[0] -= n * s_list[i]
            imos[d] += n * s_list[i]
        elif l[i] - a >= 1:
            imos[0] -= (l[i] - a + d) // d * s_list[i]
            imos[(l[i] - a) % d] += s_list[i]
            imos[d] += (l[i] - a) // d * s_list[i]
        if r_list[i] - a >= n * d:
            imos[0] += n * s_list[i]
            imos[d] -= n * s_list[i]
        elif r_list[i] - a >= 1:
            imos[0] += (r_list[i] - a + d) // d * s_list[i]
            imos[(r_list[i] - a) % d] -= s_list[i]
            imos[d] -= (r_list[i] - a) // d * s_list[i]
    val = 0
    score = 0
    for i in range(0, d):
        val += imos[i]
        score = max(score, val)
    return score

if __name__ == '__main__':
    main()
