from common import read_file
from enum import Enum, auto



class Direction(Enum):
    UP = auto()
    DOWN = auto()
    LEFT = auto()
    RIGHT = auto()


def move(direction, position, recursion=0):
    global grid, all_coords
    if recursion > 650:
        return
    all_coords.add(position)
    x, y = position
    max_x = len(grid[0]) - 1
    max_y = len(grid) - 1
    new_position = (0, 0)
    match direction:
        case Direction.UP:
            new_position = (x, y-1)
        case Direction.DOWN:
            new_position = (x, y+1)
        case Direction.LEFT:
            new_position = (x-1, y)
        case Direction.RIGHT:
            new_position = (x+1, y)
    new_x, new_y = new_position
    if new_x < 0 or new_x > max_x or new_y < 0 or new_y > max_y:
        return
    if grid[new_y][new_x] == '/':
        match direction:
            case Direction.UP:
                move(Direction.RIGHT, (new_x, new_y), recursion=recursion+1)
            case Direction.DOWN:
                move(Direction.LEFT, (new_x, new_y), recursion=recursion+1)
            case Direction.LEFT:
                move(Direction.DOWN, (new_x, new_y), recursion=recursion+1)
            case Direction.RIGHT:
                move(Direction.UP, (new_x, new_y), recursion=recursion+1)
    elif grid[new_y][new_x] == '\\':
        match direction:
            case Direction.UP:
                move(Direction.LEFT, (new_x, new_y), recursion=recursion+1)
            case Direction.DOWN:
                move(Direction.RIGHT, (new_x, new_y), recursion=recursion+1)
            case Direction.LEFT:
                move(Direction.UP, (new_x, new_y), recursion=recursion+1)
            case Direction.RIGHT:
                move(Direction.DOWN, (new_x, new_y), recursion=recursion+1)
    elif grid[new_y][new_x] == '|':
        match direction:
            case Direction.LEFT | Direction.RIGHT:
                move(Direction.UP, (new_x, new_y), recursion=recursion+1)
                move(Direction.DOWN, (new_x, new_y), recursion=recursion+1)
            case Direction.UP | Direction.DOWN:
                move(direction, (new_x, new_y), recursion=recursion+1)
    elif grid[new_y][new_x] == '-':
        match direction:
            case Direction.UP | Direction.DOWN:
                move(Direction.LEFT, (new_x, new_y), recursion=recursion+1)
                move(Direction.RIGHT, (new_x, new_y), recursion=recursion+1)
            case Direction.LEFT | Direction.RIGHT:
                move(direction, (new_x, new_y), recursion=recursion+1)
    else:
        move(direction, (new_x, new_y), recursion=recursion+1)

if __name__ == '__main__':
    grid = read_file('input/input16.txt')

    all_coords = set()
    move(Direction.DOWN, (0, 0), 0)
    print(len(all_coords))