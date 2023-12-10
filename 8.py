from common import read_file
import re
from collections import namedtuple
from itertools import cycle

def parse(filename):
    lines = read_file(filename)

    instructions = lines[0]
    nodes = map(lambda x: re.findall('\w+', x), lines[2:])

    n = namedtuple('Node', ['L', 'R'])
    g = {node: n(L, R) for node, L, R in nodes}
    return instructions, g


def count_steps(instructions, graph):
    i = 0
    counter = 0
    node = 'AAA'
    while node != 'ZZZ':
        if instructions[i] == 'L':
            node = graph[node].L
        else:
            node = graph[node].R
        counter += 1
        i = (i + 1) % len(instructions)
    return counter

def count_steps2(instructions, graph):
    direction = cycle(instructions)
    counter = 0
    nodes = [node for node in graph.keys() if node[2] == 'A']
    while not all(node[2] == 'Z' for node in nodes):
        for i, node in enumerate(nodes):
            nodes[i] = graph[node].L if next(direction) == 'L' else graph[node].R
        counter += 1
        if counter % 1000000 == 0:
            print(counter)
    return counter

if __name__ == '__main__':
    instructions, graph = parse('input/input8.txt')
    print(count_steps(instructions, graph))
    print(count_steps2(instructions, graph))