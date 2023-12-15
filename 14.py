import array

from common import read_file


def transpose(matrix):
    return list(zip(*matrix))


def rotate(matrix):
    return [array.array('b', row) for row in zip(*matrix)][::-1]


def rotate360(matrix):
    m = matrix
    for _ in range(4):
        for line in m:
            move(line)
        m = rotate(m)
    return m


def to_array(line):
    mapper = {'O': 1, '.': 0, '#': -1}
    return array.array('b', map(lambda x: mapper[x], line))


def print_transposed(transposed_array):
    t = transpose(transposed_array)
    for row in t:
        for num in row:
            if num == 1:
                print('O', end='')
            elif num == 0:
                print('.', end='')
            elif num == -1:
                print('#', end='')
        print()


def count_rocks(lines, transformed):
    original = 0
    trans = 0
    for line in lines:
        original += line.count('O')
    print(original)

    for line in transformed:
        trans += line.count(1)
    print(trans)


def count_load(line):
    return sum(_ for _, num in enumerate(reversed(line), 1) if num == 1)


def move(line: array.array):
    lowest_index = line.index(0)
    for i in range(lowest_index, len(line)):
        if line[i] == 1:
            if lowest_index < i:
                line[lowest_index] = 1
                line[i] = 0
                lowest_index += 1
        elif line[i] == -1:
            try:
                lowest_index = line.index(0, i)
            except ValueError:
                lowest_index = len(line) + 1
                continue
        elif line[i] == 0:
            lowest_index = min(lowest_index, i)


lines = read_file('input/input14.txt')
transposed_lines = [to_array(line) for line in transpose(lines)]
part1 = transposed_lines.copy()
s = 0

for line in part1:
    move(line)

print(sum(count_load(line) for line in part1))

# solution found by analyzing remainders
with open('loads.dat', 'w') as f:
    for i in range(1, 1_000 + 1):
        transposed_lines = rotate360(transposed_lines)
        load = sum(count_load(line) for line in transposed_lines)
        print(f'{i},{load}', file=f)
