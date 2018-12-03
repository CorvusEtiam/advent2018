import numpy as np 
import re 


with open("/home/misiek/Project/rust/advent2018/inputs/input3.txt", 'r') as f:
    inp = []
    for r in f.readlines():
        r = re.split('[^0-9]+', r[1:].strip())
        inp.append([int(d) for d in r])

fabric = np.zeros((1000, 1000))

def part1():
    for n, x, y, dx, dy in inp:
        fabric[x:x+dx, y:y+dy] += 1
    return np.sum(fabric > 1)

def part2():
    for n, x, y, dx, dy in inp:
        if np.all(fabric[x:x+dx, y:y+dy] == 1):
            return n

print(part1())
print(part2())