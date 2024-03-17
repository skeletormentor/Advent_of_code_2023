from collections import deque

from common import read_file


def find_start(lines):
    global rows, cols
    for i in range(rows):
        for j in range(cols):
            if lines[i][j] == 'S':
                return i, j


def is_valid(row, col):
    try:
        if lines[row][col] != '#':
            return True
    except IndexError:
        return False


def bfs(row, col, distance):
    global lines, distances, dx, dy
    q = deque()
    q.append((row, col))

    dist = 0
    while len(q) > 0 and dist < distance:
        dist += 1
        y, x = q.popleft()
        for i in range(4):
            adj_x = x + dx[i]
            adj_y = y + dy[i]
            if is_valid(adj_y, adj_x):
                q.append((adj_y, adj_x))
                distances[adj_y][adj_x].add(dist)
                bfs(adj_y, adj_x, dist - 1)


if __name__ == '__main__':
    lines = read_file('input/input21.test')

    rows = len(lines)
    cols = len(lines[0])
    distances = [[set() for x in range(cols)] for y in range(rows)]

    dx = (0, 1, 0, -1)
    dy = (-1, 0, 1, 0)

    bfs(*find_start(lines), 6)
    for d in distances:
        print(d)
