from common import read_file
import re


def distance_between(p1, p2):
    return abs(p2[1] - p1[1]) + abs(p2[0] - p1[0])


def transpose(matrix):
    return list(zip(*matrix))


def get_emptys(lines):
    rows = []
    cols = []
    t = transpose(lines)
    for i, line in enumerate(lines):
        if '#' not in line:
            rows.append(i)
    for i, line in enumerate(t):
        if '#' not in line:
            cols.append(i)
    return rows, cols


def find_galaxies(lines):
    galaxies = []
    regex = re.compile('#')
    for y, line in enumerate(lines):
        for match in regex.finditer(''.join(line)):
            x = match.span()[0]
            galaxies.append((x, y))
    return galaxies


def sum_all_distances(galaxies):
    sum_distances = 0
    calculated = set()
    for galaxy1 in galaxies:
        for galaxy2 in galaxies:
            if (galaxy1, galaxy2) not in calculated:
                distance = distance_between(galaxy1, galaxy2)
                sum_distances += distance
            calculated.update([(galaxy1, galaxy2), (galaxy2, galaxy1)])
    return sum_distances


def expand_galaxy(galaxy, empty_rows, empty_cols, spaces=1):
    x, y = galaxy
    if spaces != 1:
        spaces -= 1
    new_x = x + len([ec for ec in empty_cols if ec < x]) * spaces
    new_y = y + len([er for er in empty_rows if er < y]) * spaces
    return new_x, new_y


def expanded_galaxies(lines, spaces=1):
    galaxies = find_galaxies(lines)
    empty_rows, empty_cols = get_emptys(lines)
    expanded = []
    for galaxy in galaxies:
        expanded.append(expand_galaxy(galaxy, empty_rows, empty_cols, spaces=spaces))
    return expanded


if __name__ == '__main__':
    _lines = read_file('input/input11.txt')
    galaxies1 = expanded_galaxies(_lines)
    galaxies2 = expanded_galaxies(_lines, spaces=1_000_000)
    print(sum_all_distances(galaxies1))
    print(sum_all_distances(galaxies2))
