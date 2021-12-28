BLACK = 1
WHITE = 2

def main():
    br, bc, wr, wc = map(int, input().split())
    black = Position(br, bc)
    white = Position(wr, wc)

    while True:
        mr, mc = map(int, input().split())
        if mr == 0 and mc == 0:
            break
        else:
            mole = Position(mr, mc)
            cost_black = mole.calc_cost(black)
            cost_white = mole.calc_cost(white)
            if cost_black < cost_white:
                black = mole
                print(BLACK)
            else:
                white = mole
                print(WHITE)

class Position:
    def __init__(self, r, c) -> None:
        self.r = r
        self.c = c
    def calc_cost(self, another) -> int:
        return abs(self.r - another.r) + abs(self.c - another.c)

if __name__ == '__main__':
    main()