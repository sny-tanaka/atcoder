package main

import (
	"bufio"
    "fmt"
	"os"
	"strconv"
)

var sc = bufio.NewScanner(os.Stdin)

func nextInt() int {
    sc.Scan()
    i, e := strconv.Atoi(sc.Text())
    if e != nil {
        panic(e)
    }
    return i
}

func main() {
    sc.Split(bufio.ScanWords)
    n := nextInt()
	m := nextInt()
	d := nextInt()
	r := make([]int, m + 1)
    for i := 0; i < m + 1; i++ {
        r[i] = nextInt()
    }
	s := make([]int, m)
	for i := 0; i < m; i++ {
        s[i] = nextInt()
    }
    fmt.Println(solve(n, m, d, r, s))
}

func solve(n int, m int, d int, r []int, s []int) int {
	a := -(n * d / 2)
	tmp_sl := make([]int, 2 * m)
	for i := 0; i < m; i++ {
        tmp_sl[i] = -r[m - i]
    }
	for i := 1; i <= m; i++ {
        tmp_sl[m + i - 1] = r[i] + 1
    }
	l := tmp_sl[:len(tmp_sl) - 1]
	r_sl := tmp_sl[1:]
	s_sl := make([]int, 2 * m - 1)
	for i := 0; i < m; i++ {
        s_sl[i] = s[m - i - 1]
    }
	for i := 1; i < m; i++ {
        s_sl[m + i - 1] = s[i]
    }
	imos := make([]int, d + 1)
	for i := 0; i < 2 * m - 1; i++ {
        if l[i] - a >= n * d {
            imos[0] -= n * s_sl[i]
            imos[d] += n * s_sl[i]
        } else if l[i] - a >= 1 {
            imos[0] -= (l[i] - a + d) / d * s_sl[i]
            imos[(l[i] - a) % d] += s_sl[i]
            imos[d] += (l[i] - a) / d * s_sl[i]
        }
        if r_sl[i] - a >= n * d {
            imos[0] += n * s_sl[i]
            imos[d] -= n * s_sl[i]
        } else if r_sl[i] - a >= 1 {
            imos[0] += (r_sl[i] - a + d) / d * s_sl[i]
            imos[(r_sl[i] - a) % d] -= s_sl[i]
            imos[d] -= (r_sl[i] - a) / d * s_sl[i]
        }
    }
	val := 0
	score := 0
	for i := 0; i < d; i++ {
        val += imos[i]
		if val > score {
			score = val
		}
	}
	return score
}
