import itertools as it
from functools import reduce
import operator

def prod(xs):
    return reduce(operator.mul, xs, 1)

def solve(xs, size):
    return next((prod(t) for t in it.combinations(xs, size) if sum(t) == 2020))

inp = open('input/2020-1.in', 'r').readlines()
xs = [int(x.strip()) for x in inp]
s1 = solve(xs, 2)
s2 = solve(xs, 3)

print(s1,s2)
