from functools import reduce

def flatten(ls):
    return reduce(list.__add__, ls)

ls = []
with open('in.txt', 'r') as f:
    ls = f.read().split('\n')
    ls = [x.split(" ") for x in ls]
    ls = flatten(ls)
    ls = [x for x in ls if x.isdigit()]

with open('primes.txt', 'x') as f:
    csv = ",".join(ls)
    f.write(csv)
