import numpy as np
from shapely.geometry import Polygon, Point

from common import read_file


def get_outer_points(lines):
    x = 0
    y = 0
    xs = []
    ys = []
    coords = []
    points = set()

    for line in lines:
        direction, amount, color = line.split()
        xs.append(x)
        ys.append(y)
        coords.append((x, y))
        match direction:
            case 'R':
                for _ in range(int(amount)):
                    points.add((x, y))
                    x += 1
            case 'L':
                for _ in range(int(amount)):
                    points.add((x, y))
                    x -= 1
            case 'D':
                for _ in range(int(amount)):
                    points.add((x, y))
                    y += 1
            case 'U':
                for _ in range(int(amount)):
                    points.add((x, y))
                    y -= 1
    xs = np.array(xs)
    ys = np.array(ys)
    return xs, ys, points, coords


def get_inner_points(X, Y, coords):
    points = set()
    polygon = Polygon(coords)

    for y in range(Y.min(), Y.max() + 1):
        for x in range(X.min(), X.max() + 1):
            if polygon.contains(Point(x, y)):
                points.add((x, y))
    return points


if __name__ == '__main__':
    lines = read_file('input/input18.txt')

    X, Y, points, coords = get_outer_points(lines)
    points |= get_inner_points(X, Y, coords)
    print(len(points))
