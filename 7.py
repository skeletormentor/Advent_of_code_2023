from common import read_file
from collections import Counter, defaultdict
from functools import cmp_to_key

lines = read_file('input/input7.txt')

card = {'A': 14, 'K': 13, 'Q': 12, 'J': 11, 'T': 10, '9': 9,
        '8': 8, '7': 7, '6': 6, '5': 5, '4': 4, '3': 3, '2': 2}


def rank(cards):
    values = sorted(Counter(cards).values())
    if 5 in values:
        return 6
    elif 4 in values:
        return 5
    elif 3 in values and 2 in values:
        return 4
    elif 3 in values:
        return 3
    elif values.count(2) == 2:
        return 2
    elif values.count(2) == 1:
        return 1
    else:
        return 0


def compare(card1, card2):
    if card1 == card2:
        return 0
    for i in range(5):
        if card[card1[i]] == card[card2[i]]:
            continue
        elif card[card1[i]] > card[card2[i]]:
            return 1
        else:
            return -1


cards = defaultdict(list)
sorted_cards = []
getbid = {}

for line in lines:
    hand, bid = line.split()
    bid = int(bid)
    value = rank(hand)
    cards[value].append(hand)
    getbid[hand] = bid

for i in range(7):
    sorted_cards.extend(sorted(cards[i], key=cmp_to_key(compare)))

p1 = 0

for i, card in enumerate(sorted_cards, 1):
    p1 += i * getbid[card]
print(p1)
