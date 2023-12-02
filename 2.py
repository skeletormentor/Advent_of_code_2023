import re
from functools import reduce
from common import read_file


def parse(filename: str) -> list[list[dict[str, int]]]:
    gamelist = []
    games_texts = read_file(filename)
    for game_text in games_texts:
        bags = []
        pick_texts = game_text.split(':')[1].strip().split(';')
        for pick_text in pick_texts:
            picks = re.findall(r'(\d+) (\w+)', pick_text)
            pick = {p[1]: int(p[0]) for p in picks}
            bags.append(pick)
        gamelist.append(bags)
    return gamelist


def validate(gamelist: list[list[dict[str, int]]]) -> int:
    s = 0
    max_values = {'red': 12, 'green': 13, 'blue': 14}
    for i, game in enumerate(gamelist):
        is_valid = True
        for grab in game:
            for color, amount in grab.items():
                if amount > max_values[color]:
                    is_valid = False
        if is_valid:
            s += i + 1
    return s


def get_max(gamelist: list[list[dict[str, int]]]) -> int:
    s = 0
    for game in gamelist:
        max_values = {}
        for grab in game:
            for color, amount in grab.items():
                if color in max_values.keys():
                    if amount >= max_values[color]:
                        max_values[color] = amount
                else:
                    max_values[color] = amount
        s += reduce((lambda x, y: x * y), max_values.values())
    return s


if __name__ == '__main__':
    games = parse('input/input2.txt')
    print(validate(games))
    print(get_max(games))
