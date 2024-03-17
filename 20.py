from common import read_file
from typing import NamedTuple
from enum import Enum
from dataclasses import dataclass
from collections import deque, defaultdict

lines = read_file('input/input20.test1')
Type = Enum('Type', ['flipflop', 'broadcaster', 'conjunction'])
Status = Enum('Status', ['off', 'on'])


@dataclass
class Module:
    name: str
    sends: list[str]
    type: Type
    status: Status = Status['off']


def emptymodule():
    return Module(name="", sends=[], type=Type['broadcaster'], status=Status['off'])


modules = defaultdict(emptymodule)
for line in lines:
    type_ = Type['broadcaster']
    name, sends = line.split(' -> ')
    if line[0] == '%':
        type_ = Type['flipflop']
        name = name[1:]
    elif line[0] == '&':
        type_ = Type['conjunction']
        name = name[1:]
    sends = sends.split(', ')
    modules[name] = Module(name, sends, type_)


def push_button(module: Module):
    global modules
    sends = modules[module.name].sends
    for send in sends:
        if modules[send].type == Type['flipflop']:
            if modules[send].status == Status['off']:
                modules[send].status = Status['on']
            else:
                modules[send].status = Status['off']
        elif modules[send].type == Type['conjunction']:
            is_on = True
            for conn in modules[send].sends:
                if modules[conn].status == Status['off']:
                    is_on = False
            if is_on:
                modules[send].status = Status['off']
            else:
                modules[send].status = Status['on']
        print(f'{modules[module.name].name} -> {modules[send].name}: {modules[send].status}')
    return sends


q = deque(['broadcaster'])
while len(q) != 0:
    module = q.pop()
    q.extendleft(push_button(modules[module]))
