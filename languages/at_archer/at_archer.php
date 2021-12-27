<?php
class Scanner {
    private $arr = [];
    private $count = 0;
    private $pointer = 0;
    public function next() {
        if($this->pointer >= $this->count) {
            $str = trim(fgets(STDIN));
            $this->arr = explode(' ', $str);
            $this->count = count($this->arr);
            $this->pointer = 0;
        }
        $result = $this->arr[$this->pointer];
        $this->pointer++;
        return $result;
    }
    public function hasNext() {
        return $this->pointer < $this->count;
    }
    public function nextInt() {
        return (int)$this->next();
    }
    public function nextDouble() {
        return (double)$this->next();
    }
}

function intFloor(float $val) {
    return (int)floor($val);
}

function main() {
    $sc = new Scanner();
    $n = $sc->nextInt();
    $m = $sc->nextInt();
    $d = $sc->nextInt();
    $r = [];
    for($i = 0; $i < $m + 1; $i++) {
        $r[$i] = $sc->nextInt();
    }
    $s = [];
    for($i = 0; $i < $m; $i++) {
        $s[$i] = $sc->nextInt();
    }
    print_r(solve($n, $m, $d, $r, $s));
}

function solve(int $n, int $m, int $d, array $r, array $s) {
    $a = -intFloor($n * $d / 2);
    $tmpList = array_merge(array_map(fn($x): int => -$x, array_slice(array_reverse($r), 0, $m)), array_map(fn($x): int => $x + 1, array_slice($r, 1)));
    $l = array_slice($tmpList, 0, 2 * $m);
    $rList = array_slice($tmpList, 1);
    $sList = array_merge(array_reverse($s), array_slice($s, 1));
    $imos = array_fill(0, $d + 1, 0);
    for($i = 0; $i < 2 * $m - 1; $i++) {
		if($l[$i] - $a >= $n * $d) {
			$imos[0] -= $n * $sList[$i];
			$imos[$d] += $n * $sList[$i];
		}
		else if($l[$i] - $a >= 1) {
			$imos[0] -= intFloor(($l[$i] - $a + $d) / $d) * $sList[$i];
			$imos[($l[$i] - $a) % $d] += $sList[$i];
			$imos[$d] += intFloor(($l[$i] - $a) / $d) * $sList[$i];
		}
		if($rList[$i] - $a >= $n * $d) {
			$imos[0] += $n * $sList[$i];
			$imos[$d] -= $n * $sList[$i];
		}
		else if($rList[$i] - $a >= 1) {
			$imos[0] += intFloor(($rList[$i] - $a + $d) / $d) * $sList[$i];
			$imos[($rList[$i] - $a) % $d] -= $sList[$i];
			$imos[$d] -= intFloor(($rList[$i] - $a) / $d) * $sList[$i];
		}
    }
    $val = 0;
    $score = 0;
    for($i = 0; $i < $d; $i++) {
        $val += $imos[$i];
        if($val > $score) {
            $score = $val;
        }
    }
    return $score;
}

main();
?>