import numpy as np
import itertools

def main():
    n = int(input())
    xy = []

    for _ in range(n):
        xy.append(np.array(list(map(int, input().split(" ")))))

    cnt = 0
    for (i, j, k) in itertools.combinations(range(n), 3):
        o = xy[i]
        a = xy[j]
        b = xy[k]

        oa = o - a
        ob = o - b
        s = oa[0] * ob[1] - oa[1] * ob[0]
        if s == 0:
            continue
        cnt += 1

    print(cnt)
    
if __name__ == "__main__":
    main()
