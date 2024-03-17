from enum import Enum
from typing import NamedTuple

Category = Enum('Category', 'x m a s')


class Rule(NamedTuple):
    assign: str
    value: int = None
    symbol: str = None
    category: Category = None


workflows = {}
ratings = []


with open('input/input19.txt') as f:
    workflows_, ratings_ = f.read().split('\n\n')
    for workflow in workflows_.split('\n'):
        name_idx = workflow.find('{')
        name = workflow[:name_idx]
        rules = workflow[name_idx + 1:-1].split(',')
        rule_list = []
        for r in rules:
            if '>' in r:
                category = r[:r.find('>')]
                value = int(r[r.find('>') + 1:(r.find(':'))])
                assign = r[(r.find(':') + 1):]
                rule_list.append(Rule(assign, symbol='>', category=Category[category], value=value))
            elif '<' in r:
                category = r[:r.find('<')]
                value = int(r[r.find('<') + 1:(r.find(':'))])
                assign = r[(r.find(':') + 1):]
                rule_list.append(Rule(assign, symbol='<', category=Category[category], value=value))
            else:
                rule_list.append(Rule(r[(r.find(':') + 1):]))
        workflows[name] = rule_list

    for ratingset_ in ratings_[:-1].split('\n'):
        ratingset = {}
        for rating in ratingset_[1:-1].split(','):
            ratingset[Category[rating[0]]] = int(rating[2:])
        ratings.append(ratingset)


def validate(rating, name='in'):
    global workflows
    if name == 'A':
        return True
    elif name == 'R':
        return False
    is_valid = False
    rules = workflows[name]
    for rule in rules:
        if rule.value:
            if rule.symbol == '<':
                if rating[rule.category] < rule.value:
                    return validate(rating, name=rule.assign)
                else:
                    continue
            elif rule.symbol == '>':
                if rating[rule.category] > rule.value:
                    return validate(rating, name=rule.assign)
                else:
                    continue
        else:
            return validate(rating, name=rule.assign)
    return is_valid


s = 0
for rating in ratings:
    if validate(rating):
        s += sum(rating.values())
print(s)
