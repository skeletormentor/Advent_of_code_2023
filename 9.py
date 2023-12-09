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

    return seqs[-1][-1]

def add_histories(seqs):
    return sum(extrapolate(s) for s in seqs)

if __name__ == '__main__':
    sequences = parse('input/input9.txt')
    print(add_histories(sequences))