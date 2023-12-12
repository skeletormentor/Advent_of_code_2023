from collections import defaultdict, deque

from common import read_file


def adjacent_coordinates(coord, current_char, up_char, down_char, left_char, right_char):
    def check_dir(directions, dir_str):
        map_directions = {'up': '|7F', 'down': '|LJ', 'left': '-LF', 'right': '-J7'}
        valid_directions = []
        for direction, s in zip(directions, dir_str):
            if direction[0] in map_directions[s]:
                valid_directions.append(direction[1])
        return valid_directions

    x, y = coord
    up_char = [up_char, (x, y - 1)]
    down_char = [down_char, (x, y + 1)]
    left_char = [left_char, (x - 1, y)]
    right_char = [right_char, (x + 1, y)]

    match current_char:
        case '|':
            return check_dir([up_char, down_char], ['up', 'down'])
        case '-':
            return check_dir([left_char, right_char], ['left', 'right'])
        case 'L':
            return check_dir([up_char, right_char], ['up', 'right'])
        case 'J':
            return check_dir([up_char, left_char], ['up', 'left'])
        case '7':
            return check_dir([down_char, left_char], ['down', 'left'])
        case 'F':
            return check_dir([down_char, right_char], ['down', 'right'])


def bfs(start):
    global g, dist, visited
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


def generate_graph(lines, start_char='|'):
    global size
    graph = defaultdict(set)
    _visited = {}
    _dist = {}
    start = (0, 0)
    for y, row in enumerate(lines):
        for x, char in enumerate(row):
            if char == 'S':
                start = (x, y)
                char = start_char
            up, down, left, right = '.', '.', '.', '.'
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
                _visited[(x, y)] = False
                _dist[(x, y)] = 0
                graph[(x, y)].update(neighbors)
                for neighbor in neighbors:
                    graph[neighbor].add((x, y))
    return graph, _visited, _dist, start


def get_largest(_dist):
    global size
    largest = 0
    for y in range(size[1]):
        for x in range(size[0]):
            if (x, y) in _dist.keys() and _dist[(x, y)] > 0:
                largest = max(largest, _dist[(x, y)])
    return largest


if __name__ == '__main__':
    _lines = read_file('input/input10.txt')
    size = len(_lines), len(_lines[0])
    g, visited, dist, S = generate_graph(_lines)

    bfs(S)

    print(get_largest(dist))
