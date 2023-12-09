from common import read_file

def parse(filename):
    lines = read_file(filename)
    return [[int(i) for i in line.split()] for line in lines]

def extrapolate(seq):
    length = len(seq) - 1
    temp_seq = []
    seqs = []
    while True:
        for i in range(length):
            temp_seq.append(seq[i+1] - seq[i])
        length -= 1
        seqs.append(seq)
        seq = temp_seq
        if sum(seq) == 0:
            break
        temp_seq = []

    seqs.reverse()

    for i in range(len(seqs) - 1):
        seqs[i+1].append(seqs[i][-1] + seqs[i+1][-1])
        seqs[i+1].insert(0, seqs[i+1][0] - seqs[i][0])

    return seqs[-1][0], seqs[-1][-1]

def add_histories(seqs):
    p1 = 0
    p2 = 0
    for s in seqs:
        a, b = extrapolate(s)
        p2 += a
        p1 += b
    return p1, p2

if __name__ == '__main__':
    sequences = parse('input/input9.txt')
    [print(s) for s in add_histories(sequences)]