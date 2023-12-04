from common import read_file
import re


def calculate_points(filename):
    lines = read_file(filename)
    splits = [line.split('|') for line in lines]
    all_points = 0
    number_of_wins = []
    regex = re.compile(r'\d+')
    for i, (winning_numbers, hand_numbers) in enumerate(splits):
        wins = set(regex.findall(winning_numbers)[1:])
        hand = set(regex.findall(hand_numbers))
        points = len(hand.intersection(wins))
        number_of_wins.append(points)
        if points > 0:
            all_points += 2 ** (points - 1)
    return all_points, number_of_wins


def traverse(wins):
    cards = [1] * len(wins)
    for i, win in enumerate(wins):
        for j in range(win):
            for _ in range(cards[i]):
                cards[i+j+1] += 1
    return sum(cards)


if __name__ == '__main__':
    p, w = calculate_points('input/test_input_4.txt')
    print(p)
    print(traverse(w))
