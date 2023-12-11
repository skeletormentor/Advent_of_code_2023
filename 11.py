from common import read_file
import re

def distance(point1, point2):
    return abs(point2[1] - point1[1]) + abs(point2[0] - point1[0])

lines = read_file('input/input11.txt')

galaxies = []

def transpose(M):
    return list(zip(*M))

def expand(lines):
    expanded = []
    width = len(lines[0])
    for line in lines:
        if '#' not in line:
            expanded.append('.' * width)
        expanded.append(line)
    return expanded

def full_expand(lines):
    expanded = expand(lines)
    t = transpose(expanded)
    expanded = expand(t)
    t = transpose(expanded)
    return t

for y, line in enumerate(full_expand(lines)):
    for match in re.finditer('#', ''.join(line)):
        x = match.span()[0]
        galaxies.append((x, y))

s = 0
calculated = set()
for galaxy1 in galaxies:
    for galaxy2 in galaxies:
        if (galaxy1, galaxy2) not in calculated:    
            d = distance(galaxy1, galaxy2)
            s += d
        calculated.update([(galaxy1, galaxy2), (galaxy2, galaxy1)])

print(s)