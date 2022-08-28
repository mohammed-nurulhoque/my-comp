v = []
for _2 in range(7):
    for _3 in range(4):
        for _5 in range(3):
            for _7 in range(2):
                for _11 in range(2):
                    bases = [2, 3, 5, 7, 11]
                    exps  = [_2, _3, _5, _7, _11]
                    n = sum(map(lambda i: bases[i] ** exps[i], range(5)))
                    if n == 1 or n > 2000000000: continue
                    phi = 2**(_2-1) * 2*3**(_3-1) * 4*5**(_5-1) * 6*7**(_7-1) * 10*11**(_11-1)
                    v.append((n, phi, phi/(n-1)))
print(v)