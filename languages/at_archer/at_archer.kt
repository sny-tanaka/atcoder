import java.io.PrintWriter

fun main(args : Array<String>) {
    val pw = PrintWriter(System.out)
    val (n, m, d) = readLine()!!.split(" ").map{ it.toLong() }
    val r = readLine()!!.split(" ").map{ it.toLong() }
    val s = readLine()!!.split(" ").map{ it.toLong() }
    pw.println(solve(n, m, d, r, s))
    pw.flush()
}

fun solve(n: Long, m: Long, d: Long, r: List<Long>, s: List<Long>): Long {
    val a = -(n * d / 2)
    val tmp_list = r.asReversed().subList(0, m.toInt()).map { -it }.plus(r.subList(1, m.toInt() + 1).map { it + 1 })
    val l = tmp_list.subList(0, 2 * m.toInt() - 1)
    val r_list = tmp_list.subList(1, 2 * m.toInt())
    val s_list = s.asReversed().plus(s.subList(1, m.toInt()))
    var imos = Array(d.toInt() + 1) { 0L }
    for(i in 0 until 2 * m.toInt() - 1) {
        if(l[i] - a >= n * d) {
            imos[0] -= n * s_list[i];
            imos[d.toInt()] += n * s_list[i];
        } else if(l[i] - a >= 1) {
            imos[0] -= (l[i] - a + d) / d * s_list[i];
            imos[((l[i] - a) % d).toInt()] += s_list[i];
            imos[d.toInt()] += (l[i] - a) / d * s_list[i];
        }
        if(r_list[i] - a >= n * d) {
            imos[0] += n * s_list[i];
            imos[d.toInt()] -= n * s_list[i];
        } else if(r_list[i] - a >= 1) {
            imos[0] += (r_list[i] - a + d) / d * s_list[i];
            imos[((r_list[i] - a) % d).toInt()] -= s_list[i];
            imos[d.toInt()] -= (r_list[i] - a) / d * s_list[i];
        }
    }
    var value = 0L
    var score = 0L
    for(i in 0 until d.toInt()) {
        value += imos[i]
        if(value > score) {
            score = value
        }
    }
    return score
}