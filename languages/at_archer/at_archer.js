"use strict";

const main = arg => {
  const input = arg.trim().split("\n");

  const n = parseInt(input[0].split(" ")[0]);
  const m = parseInt(input[0].split(" ")[1]);
  const d = parseInt(input[0].split(" ")[2]);
  const r = input[1].split(" ").map(x=>parseInt(x));
  const s = input[2].split(" ").map(x=>parseInt(x));
  console.log(solve(n, m, d, r, s));
}

let solve = function(n, m, d, r, s) {
  const a = -Math.floor(n * d / 2);
  const tmpr = r.map(x => x + 1).slice(1);
  const tmpList = r.reverse().map(x => -x).slice(0, -1).concat(tmpr);
  const l = tmpList.slice(0, -1);
  const rList = tmpList.slice(1);
  const tmps = s.slice(1);
  const sList = s.reverse().concat(tmps);
  const imos = Array(d + 1);
  imos.fill(0);
  for(let i = 0; i < 2 * m - 1; i++) {
    if(l[i] - a >= n * d) {
      imos[0] -= n * sList[i];
      imos[d] += n * sList[i];
    } else if(l[i] - a >= 1) {
      imos[0] -= Math.floor((l[i] - a + d) / d) * sList[i];
      imos[(l[i] - a) % d] += sList[i];
      imos[d] += Math.floor((l[i] - a) / d) * sList[i];
    }
    if(rList[i] - a >= n * d) {
      imos[0] += n * sList[i];
      imos[d] -= n * sList[i];
    } else if(rList[i] - a >= 1) {
      imos[0] += Math.floor((rList[i] - a + d) / d) * sList[i];
      imos[(rList[i] - a) % d] -= sList[i];
      imos[d] -= Math.floor((rList[i] - a) / d) * sList[i];
    }
  }
  let val = 0;
  let score = 0;
  for(let i = 0; i < d; i++) {
    val += imos[i];
    score = Math.max(score, val);
  }
  return score;
}

main(require('fs').readFileSync('/dev/stdin', 'utf8'));
