from common import read_file
from more_itertools import distinct_permutations
import re
from typing import NamedTuple, Iterator


class Configuration(NamedTuple):
    springs: str
    groups: list[int]
    n_of_unknown: int
    n_of_springs: int
    n_of_all_springs: int
    unknown_locations: list[int]

    def _generate_permutations(self) -> Iterator[tuple[str, ...]]:
        springs = self.n_of_all_springs - self.n_of_springs
        unknowns = self.n_of_unknown - springs
        permutations = distinct_permutations(springs * '#' + unknowns * '.', self.n_of_unknown)
        return permutations

    def generate_strings(self) -> list[str]:
        s = list(self.springs)
        springs = []
        for perm in self._generate_permutations():
            for char, i in zip(perm, self.unknown_locations):
                s[i] = char
            springs.append(''.join(s))
        return springs

    def generate_pattern(self) -> re.Pattern:
        pattern = r'\.*'
        for i, location in enumerate(self.groups):
            pattern += r'#{' + str(location) + r'}\.'
            if i == len(self.groups) - 1:
                pattern += '*'
            else:
                pattern += '+'
        return re.compile(pattern)


def parse(filename: str) -> list[Configuration]:
    lines = read_file(filename)
    configurations = []
    for line in lines:
        springs, groups = line.split()
        groups = [int(val) for val in groups.split(',')]
        n_of_all_springs = sum(groups)
        n_of_unknown = springs.count('?')
        n_of_springs = springs.count('#')
        unknown_locations = [f.span()[0] for f in re.finditer(r'\?', springs)]
        configurations.append(Configuration(springs, groups, n_of_unknown,
                                            n_of_springs, n_of_all_springs, unknown_locations))
    return configurations


def validate(configurations: list[Configuration]) -> int:
    all_valid = 0
    for conf in configurations:
        generated = conf.generate_strings()
        valid = 0
        for s in generated:
            match = conf.generate_pattern().match(s)
            if match:
                valid += 1
        all_valid += valid
    return all_valid


if __name__ == '__main__':
    confs = parse('input/input12.txt')
    print(validate(confs))
