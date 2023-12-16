import re


def read_file(filename):
    with open(filename) as f:
        return [row.rstrip('\n') for row in f.read().split(',')]


def hashing(input_string):
    current_value = 0
    for char in input_string:
        current_value += ord(char)
        current_value *= 17
        current_value %= 256
    return current_value


def first_part(lines):
    return sum(hashing(line) for line in lines)


def operate(lines):
    boxes = [[] for _ in range(256)]
    pattern = re.compile(r'(\w+)([-=])(\d)?')
    focal_lengths = {}
    for line in lines:
        label, action, focal_lenght = pattern.match(line).groups()
        box_index = hashing(label)
        if action == '-':
            if label in boxes[box_index]:
                boxes[box_index].remove(label)
        elif action == '=':
            if label not in boxes[box_index]:
                boxes[box_index].append(label)
            focal_lengths[label] = int(focal_lenght)
    return boxes, focal_lengths


def calculate_focusing_power(boxes, focal_lengths):
    s = 0
    for box_index, box in enumerate(boxes, 1):
        if box:
            for slot, label in enumerate(box, 1):
                s += box_index * slot * focal_lengths[label]
    return s


if __name__ == '__main__':
    lines = read_file('input/input15.txt')
    example = 'rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7'.split(',')

    print(first_part(lines))
    boxes, fl = operate(lines)
    print(calculate_focusing_power(boxes, fl))
