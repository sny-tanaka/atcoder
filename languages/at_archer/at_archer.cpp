#include <vector>
#include <iostream>
#include <algorithm>
#include <iterator>
using namespace std;

long long solve(long long n, int m, long long d, const vector<long long>& r, const vector<long long>& s) {
	long long a = -(n * d / 2);
    vector<long long> tmp_vec1, tmp_vec2, l, r_vec, s_vec;
    reverse_copy(r.begin() + 1, r.end(), back_inserter(tmp_vec1));
    for_each(tmp_vec1.begin(), tmp_vec1.end(), [](long long& element) {
        element *= -1;
    });
    copy(r.begin() + 1, r.end(), back_inserter(tmp_vec2));
    for_each(tmp_vec2.begin(), tmp_vec2.end(), [](long long& element) {
        ++element;
    });
    copy(tmp_vec2.begin(), tmp_vec2.end(), back_inserter(tmp_vec1));
    copy(tmp_vec1.begin(), tmp_vec1.end() - 1, back_inserter(l));
    copy(tmp_vec1.begin() + 1, tmp_vec1.end(), back_inserter(r_vec));
    reverse_copy(s.begin(), s.end(), back_inserter(s_vec));
    copy(s.begin() + 1, s.end(), back_inserter(s_vec));

	vector<long long> imos(d + 1);
	for (int i = 0; i < 2 * m - 1; ++i) {
		if (l[i] - a >= n * d) {
			imos[0] -= n * s_vec[i];
			imos[d] += n * s_vec[i];
		}
		else if (l[i] - a >= 1) {
			imos[0] -= (l[i] - a + d) / d * s_vec[i];
			imos[(l[i] - a) % d] += s_vec[i];
			imos[d] += (l[i] - a) / d * s_vec[i];
		}
		if (r_vec[i] - a >= n * d) {
			imos[0] += n * s_vec[i];
			imos[d] -= n * s_vec[i];
		}
		else if (r_vec[i] - a >= 1) {
			imos[0] += (r_vec[i] - a + d) / d * s_vec[i];
			imos[(r_vec[i] - a) % d] -= s_vec[i];
			imos[d] -= (r_vec[i] - a) / d * s_vec[i];
		}
	}
	long long val = 0, score = 0;
	for (int i = 0; i < d; ++i) {
		val += imos[i];
		score = max(score, val);
	}
	return score;
}

int main() {
	long long n, m, d;
    cin >> n >> m >> d;
	vector<long long> r(m + 1), s(m);
	for (int i = 0; i < m + 1; ++i) {
		cin >> r[i];
	}
	for (int i = 0; i < m; ++i) {
		cin >> s[i];
	}
	long long answer = solve(n, m, d, r, s);
	cout << answer << endl;
	return 0;
}
