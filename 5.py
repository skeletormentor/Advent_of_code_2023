import re
import sys
from collections import namedtuple, defaultdict
from common import read_file


def mapper(maps, i, id_):
    for map_ in maps[id_]:
        if map_.source_min <= i <= map_.source_max:
            return i + map_.dest_min - map_.source_min
    return i


def get_lowest(seeds, maps):
    lowest = sys.maxsize
    for seed in seeds:
        val = mapper(maps, seed, 0)
        for i in range(1, len(maps)):
            val = mapper(maps, val, i)
            if i == len(maps) - 1 and val < lowest:
                lowest = val
    return lowest


def parse_input(filename):
    map_ = namedtuple('Map', ['dest_min', 'source_min', 'source_max'])
    lines = read_file(filename)
    id_ = -1
    regex = re.compile(r'\d+')
    seeds = [int(digit) for digit in regex.findall(lines[0])]
    maps = defaultdict(list)
    for line in lines[1:]:
        if not line:
            id_ += 1
        digits = regex.findall(line)
        if digits:
            dest_min, source_min, range_ = [int(d) for d in digits]
            source_max = source_min + range_ - 1
            maps[id_].append(map_(dest_min, source_min, source_max))
    return seeds, maps


s, m = parse_input('input/input5.txt')
print(get_lowest(s, m))
