def main
    n, m, d = gets.split(' ').map(&:to_i)
    r = gets.split(' ').map(&:to_i)
    s = gets.split(' ').map(&:to_i)
    puts solve(n, m, d, r, s)
end

def solve(n, m, d, r, s)
    a = -(n * d / 2)
    tmp_list = r.reverse.map{|x| -x}.slice(0, m) + r.map{|x| x + 1}.slice(1, m)
    l = tmp_list.slice(0, 2 * m - 1)
    r_list = tmp_list.slice(1, 2 * m - 1)
    s_list = s.reverse + s.slice(1, m - 1)
    imos = Array.new(d + 1, 0)
    for i in 0...(2 * m - 1) do
        if l[i] - a >= n * d then
            imos[0] -= n * s_list[i]
            imos[d] += n * s_list[i]
        elsif l[i] - a >= 1 then
            imos[0] -= (l[i] - a + d) / d * s_list[i]
            imos[(l[i] - a) % d] += s_list[i]
            imos[d] += (l[i] - a) / d * s_list[i]
        end
        if r_list[i] - a >= n * d then
            imos[0] += n * s_list[i]
            imos[d] -= n * s_list[i]
        elsif r_list[i] - a >= 1 then
            imos[0] += (r_list[i] - a + d) / d * s_list[i]
            imos[(r_list[i] - a) % d] -= s_list[i]
            imos[d] -= (r_list[i] - a) / d * s_list[i]
        end
    end
    val = 0
    score = 0
    for i in 0...d do
        val += imos[i]
        if val > score then
            score = val
        end
    end
    return score
end

main
