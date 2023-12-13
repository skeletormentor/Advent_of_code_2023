from sys import maxsize


def parse(filename):
    lavas = []

    with open(filename) as f:
        for pattern in f.read().split('\n\n'):
            lava = []
            for line in pattern.split('\n'):
                if line:
                    lava.append(line)
            lavas.append(lava)
    return lavas


def transpose(lava):
    return list(map(''.join, zip(*lava)))


def find_mirrorline(lava):
    mirrorline = maxsize
    found = False
    size = len(lava)
    for i in range(1, size):
        if lava[i-1] == lava[i]:
            j = i-1
            k = i
            while True:
                if lava[j] != lava[k]:
                    break
                else:
                    j -= 1
                    k += 1
                if j < 0 or k >= size:
                    mirrorline = i * 100
                    found = True
                    break
        if found:
            break
    if not found:
        mirrorline = find_mirrorline(transpose(lava)) // 100
    return mirrorline


def get_sums(lavas):
    s = 0
    for lava in lavas:
        s += find_mirrorline(lava)
    return s


if __name__ == '__main__':
    lavas = parse('input/input13.txt')
    print(get_sums(lavas))
