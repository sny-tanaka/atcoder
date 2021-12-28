import static java.lang.System.*;
import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        long n = Long.parseLong(sc.next());
        long m = Long.parseLong(sc.next());
        long d = Long.parseLong(sc.next());
        long[] r = new long[(int)(m + 1)];
        for(int i = 0; i < m + 1; i++) {
            r[i] = Long.parseLong(sc.next());
        }
        long[] s = new long[(int)m];
        for(int i = 0; i < m; i++) {
            s[i] = Long.parseLong(sc.next());
        }
        out.println(solve(n, m, d, r, s));
    }
    static long solve(long n, long m, long d, long[] r, long[] s) {
        long a = -(n * d / 2);
        long[] l = new long[(int)(2 * m - 1)];
        for(int i = 0; i < m; i ++) {
            l[i] = -r[(int)m - i];
        }
        for(int i = 0; i < m - 1; i ++) {
            l[(int)m + i] = r[i + 1] + 1;
        }
        long[] rList = new long[(int)(2 * m - 1)];
        for(int i = 0; i < m - 1; i ++) {
            rList[i] = -r[(int)m - i - 1];
        }
        for(int i = 0; i < m; i ++) {
            rList[(int)m + i - 1] = r[i + 1] + 1;
        }
        long[] sList = new long[(int)(2 * m - 1)];
        for(int i = 0; i < m - 1; i ++) {
            sList[i] = s[(int)m - i - 1];
        }
        for(int i = 0; i < m; i ++) {
            sList[(int)m + i - 1] = s[i];
        }
        long[] imos = new long[(int)d + 1];
        for(int i = 0; i < 2 * m - 1; i++) {
            if(l[i] - a >= n * d) {
                imos[0] -= n * sList[i];
                imos[(int)d] += n * sList[i];
            } else if(l[i] - a >= 1) {
                imos[0] -= (l[i] - a + d) / d * sList[i];
                imos[(int)((l[i] - a) % d)] += sList[i];
                imos[(int)d] += (l[i] - a) / d * sList[i];
            }
            if(rList[i] - a >= n * d) {
                imos[0] += n * sList[i];
                imos[(int)d] -= n * sList[i];
            } else if(rList[i] - a >= 1) {
                imos[0] += (rList[i] - a + d) / d * sList[i];
                imos[(int)((rList[i] - a) % d)] -= sList[i];
                imos[(int)d] -= (rList[i] - a) / d * sList[i];
            }
        }
        long val = 0;
        long score = 0;
        for(int i = 0; i < d; i++) {
            val += imos[i];
            if(val > score) {
                score = val;
            }
        }
        return score;
    }
}