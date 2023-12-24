from common import read_file
from collections import namedtuple, defaultdict


grid = read_file('input/input17.test')
Point = namedtuple('Point', ['x', 'y', 'value'])
g = defaultdict(set)

for y in range(len(grid)):
    for x in range(len(grid[0])):
        up, down, right, left = None, None, None, None
        if y != 0:
            up = Point(x, y-1, int(grid[y-1][x]))
        if y != len(grid) - 1:
            down = Point(x, y+1, int(grid[y+1][x]))
        if x != 0:
            left = Point(x-1, y, int(grid[y][x-1]))
        if x != len(grid[0]) - 1:
            right = Point(x+1, y, int(grid[y][x+1]))
        for neighbor in [up, down, left, right]:
            if neighbor:
                self = Point(x, y, int(grid[y][x]))
                g[self].add(neighbor)
                g[neighbor].add(self)
        # print(f'({x},{y}) {up=} {down=} {left=} {right=}')
for k, v in g.items():
    print(k, v)