from common import read_file
from collections import defaultdict, deque

lines = read_file('input/input10.txt')


def adjacent_coordinates(coord, char, up, down, left, right):
    x, y = coord
    up = [up, (x, y - 1)]
    down = [down, (x, y + 1)]
    left = [left, (x - 1, y)]
    right = [right, (x + 1, y)]

    dir = []
    match char:
        case '|':
            if up[0] in '|7F':
                dir.append(up[1])
            if down[0] in '|LJ':
                dir.append(down[1])
        case '-':
            if left[0] in '-LF':
                dir.append(left[1])
            if right[0] in '-J7':
                dir.append(right[1])
        case 'L':
            if up[0] in '|7F':
                dir.append(up[1])
            if right[0] in '-J7':
                dir.append(right[1])
        case 'J':
            if up[0] in '|7F':
                dir.append(up[1])
            if left[0] in '-LF':
                dir.append(left[1])
        case '7':
            if down[0] in '|LJ':
                dir.append(down[1])
            if left[0] in '-LF':
                dir.append(left[1])
        case 'F':
            if down[0] in '|LJ':
                dir.append(down[1])
            if right[0] in '-J7':
                dir.append(right[1])
    return dir


def bfs(start):
    queue = deque([start])
    dist[start] = 0
    while len(queue) != 0:
        node = queue.popleft()
        for neighbor in g[node]:
            if visited[neighbor]:
                continue
            queue.append(neighbor)
            visited[neighbor] = True
            dist[neighbor] = dist[node] + 1


size = (len(lines), len(lines[0]))

g = defaultdict(set)
visited = {}
dist = {}
S = (0, 0)
for y, row in enumerate(lines):
    for x, char in enumerate(row):
        if char == 'S':
            S = (x, y)
            char = '|'
        up, down, left, right = ['.', '.', '.', '.']
        if x != 0:
            left = lines[y][x - 1]
        if x != size[0] - 1:
            right = lines[y][x + 1]
        if y != 0:
            up = lines[y - 1][x]
        if y != size[1] - 1:
            down = lines[y + 1][x]
        neighbors = adjacent_coordinates((x, y), char, up, down, left, right)
        if neighbors:
            visited[(x, y)] = False
            dist[(x, y)] = 0
            g[(x, y)].update(neighbors)
            for neighbor in neighbors:
                g[neighbor].add((x, y))

bfs(S)
largest = 0
mapper = {'-': '─', '|': '│', '7': '┐', 'L': '└', 'F': '┌', 'J': '┘', 'S': '│'}
for y in range(size[1]):
    for x in range(size[0]):
        if (x, y) in dist.keys() and dist[(x, y)] > 0:
            char = lines[y][x]
            c = mapper[char]
            largest = max(largest, dist[(x, y)])
            print(c, end='')
        else:
            print('█', end='')
    print()
print(largest)
