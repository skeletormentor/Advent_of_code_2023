from common import read_file
from collections import namedtuple
import re


def parse(filename):
    lines = read_file(filename)
    xs = len(lines[0])
    ys = len(lines)
    symbols = []
    gears = []
    items = []
    item = namedtuple('Item', ['value', 'x1', 'x2', 'y'])
    for y in range(ys):
        num_matches = re.finditer(r'(\d+)', lines[y])
        for num in num_matches:
            x1, x2 = num.span()
            items.append(
                item(
                    value=int(num.group()),
                    x1=x1,
                    x2=x2,
                    y=y
                )
            )
        for x in range(xs):
            char = lines[y][x]
            if char != '.' and not char.isdigit():
                if char == '*':
                    gears.append(item('*', x, x+1, y))
                symbols.append((x, y))
    return symbols, gears, items, (xs, ys)


def get_surrounding_coords(item, size_of_table):
    max_x, max_y = size_of_table
    border = []
    # top and bottom
    for j in [item.y-1, item.y+1]:
        for i in range(item.x1 - 1, item.x2 + 1):
            if (0 <= i <= max_x - 1) and (0 <= j <= max_y - 1):
                border.append((i, j))
    # left side
    if 0 <= item.x1 - 1:
        border.append((item.x1 - 1, item.y))
    # right side
    if item.x1 <= max_x - 1:
        border.append((item.x2, item.y))
    return border


def get_sum_of_items(items, symbols, size_of_table):
    sum_of_items = 0
    for item in items:
        surr = get_surrounding_coords(item, size_of_table)
        for sym in symbols:
            if sym in surr:
                sum_of_items += item.value
    return sum_of_items


def get_gear_ratios(items, gears, size_of_table):
    gear_connections = [get_surrounding_coords(g, size_of_table) for g in gears]
    connections = set()
    # id's for gears
    for i, g in enumerate(gear_connections):
        for item in items:
            for x in range(item.x1, item.x2):
                if (x, item.y) in g:
                    connections.add((i, item.value))
    s2 = 0
    d = {}
    for c in connections:
        if c[0] in d.keys():
            s2 += c[1] * d[c[0]]
        else:
            d[c[0]] = c[1]
    return s2


if __name__ == '__main__':
    symbol_coords, gear_coords, item_coords, size = parse('input/input3.txt')

    print(get_sum_of_items(item_coords, symbol_coords, size))
    print(get_gear_ratios(item_coords, gear_coords, size))
    # 507214
    # 72553319
