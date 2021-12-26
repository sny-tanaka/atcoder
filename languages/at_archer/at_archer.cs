using System;
using System.Linq;
class Program
{
    static void Main()
    {
        var line = Console.ReadLine().Split(' ');
        long n = long.Parse(line[0]);
        long m = long.Parse(line[1]);
        long d = long.Parse(line[2]);
        long[] r = Console.ReadLine().Split(' ').Select(long.Parse).ToArray();
        long[] s = Console.ReadLine().Split(' ').Select(long.Parse).ToArray();
        Console.WriteLine(Solve(n, m, d, r, s));
    }
    static long Solve(long n, long m, long d, long[] r, long[] s)
    {
        long a = -(n * d / 2);
        var tmpIter = r.Reverse().Select(x => -x).Take((int)m).Concat(r.Select(x => x + 1).Skip(1).Take((int)m));
        var l = tmpIter.Take(2 * (int)m - 1).ToArray();
        var rArray = tmpIter.Skip(1).Take(2 * (int)m - 1).ToArray();
        var sArray = s.Reverse().Concat(s.Skip(1).Take((int)m - 1)).ToArray();
        var imos = new long[d + 1];
        for(long i = 0; i < 2 * m - 1; ++i)
        {
            if(l[i] - a >= n * d)
            {
                imos[0] -= n * sArray[i];
                imos[d] += n * sArray[i];
            }
            else if(l[i] - a >= 1)
            {
                imos[0] -= (l[i] - a + d) / d * sArray[i];
                imos[(l[i] - a) % d] += sArray[i];
                imos[d] += (l[i] - a) / d * sArray[i];
            }
            if(rArray[i] - a >= n * d)
            {
                imos[0] += n * sArray[i];
                imos[d] -= n * sArray[i];
            }
            else if(rArray[i] - a >= 1)
            {
                imos[0] += (rArray[i] - a + d) / d * sArray[i];
                imos[(rArray[i] - a) % d] -= sArray[i];
                imos[d] -= (rArray[i] - a) / d * sArray[i];
            }
        }
        long val = 0;
        long score = 0;
        for(long i = 0; i < d; ++i)
        {
            val += imos[i];
            if(val > score)
            {
                score = val;
            }
        }
        return score;
    }
}
