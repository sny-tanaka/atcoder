# Source
https://atcoder.jp/contests/xmascon21/tasks/xmascon21_i

The following is the problem sentence I translated into English.

# Problem
This problem is interactive. The scoring program uses a maximum execution time of 100ms and memory of about 16MB.
There are 2 rows and 100 columns. The cell of i-row and j-column is expressed as `(i, j)`. `(1 ≤ i ≤ 2, 1 ≤ j ≤ 100)`
There are black-rabbit and white-rabbit, and the first position is given as `(A, B)` and `(C, D)`, respectively.
Then, Whack-A-Mole begins. Answer the query online. Each query is one of the following:
 * A mole appears in the position `(e, f)`. You choose the brack or the white. The selected rabbit moves to the position where the mole is. If the rabbit moves from the position `(i, j)` to the position `(e, f)`, the cost of `∣i−e∣ + ∣j−f∣` will be incurred.
 * Finish the Whack-A-Mole. At this time, the total cost must be less than `2z + ∣A−C∣ + ∣B−D∣`, where `z` is the minimum value of the total cost when you know in advance that the position where moles appeare.
At any point in time, it is permissible that more than one of the the black, the white, and the mole are in the same position.
Also, the judge is adaptive. In other words, depending on your choice, whether or not the mole tapping ends and the mass in which the mole appears may change.

# Constrains
 * `1≤A≤2`
 * `1≤B≤100`
 * `1≤C≤2`
 * `1≤D≤100`
 * The position `(e, f)` where a mole appears must be `1≤e≤2` and `1≤f≤100`.
 * The number of queries in which moles appear is 1 or more and 10000 or less.

# Input & Output
1. The integers `A B C D` are given as the standard input separated by blanks.
2. Repeat the following.
    1. The integers `e f` are given as the standard input separated by blanks.
    2. When `e = f = 0`, it means that Whack-A-Mole is finished. Quit the program.
    3. If not, it means that a mole appears in the position `(e, f)`. Output `1` or `2` to the standard output in one line and flush the standard output. 1 stands for choosing the black, and 2 stands for choosing the white.
Judge results are undefined when input / output that does not follow this is performed (not always WA).

# Example
|  Input  |  Output  |  Explain  |
| ---- | ---- | ---- |
|  `1 12 2 24`  ||First, the black is in the position `(1, 12)`, and the white is in the position `(2, 24)`.|
| `1 1` ||A mole appears in the position `(1, 1)`.|
|| `2` |You choose the white. It costs 24. `(2, 24) => (1, 1)`|
| `2 100` ||A mole appears in the position `(2, 100)`.|
|| `1` |You choose the black. It costs 89. `(1, 12) => (2, 100)`|
| `0 0` ||Finish the Whack-A-Mole. The total cost is 113, which is `2z + ∣A−C∣ + ∣B−D∣ = 187` from `z = 87`, so the answer is correct.|
In actual judge data, it is not always possible to have a dialogue that matches this input / output example.
