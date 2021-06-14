import math
lines = [l.strip() for l in open('input.txt').readlines()]

start = int(lines[0])
idts = [(i, int(t)) for i, t in enumerate(lines[1].split(",")) if t != "x"]

best, bestid, prod = 10 ** 10, 0, 1
for _, t in idts:
    prod *= t
    if (wait := start + t - start % t) < best:
        best, bestid = wait, t
print((best - start) * bestid)

res = 0
for i, t in idts:
    p = prod // t
    res += -i * pow(p, -1, t) * p
print(res % prod)