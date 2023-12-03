import re
from itertools import chain
from math import prod

from more_itertools import windowed


def part_one(data: str) -> int:
    r = 0
    for window in list(windowed(chain([""], data.split('\n'), [""]), 3)):
        for m in re.finditer(r'\d+', window[1]):
            if any(re.search(r'[^\d.]', s[max(0, m.start() - 1):m.end() + 1]) is not None for s in window):
                r += int(m.group(0))
    return r


def part_two(data: str) -> int:
    r = 0
    for window in list(windowed(chain([""], data.split('\n'), [""]), 3)):
        for sym in re.finditer(r'\*', window[1]):
            ns = list(filter(lambda m: abs(m.start() - sym.start()) <= 1 or abs(m.end() - sym.end()) <= 1,
                             (x for s in window for x in re.finditer(r'\d+', s))))
            if len(ns) == 2:
                r += prod(map(lambda m: int(m.group()), ns))
    return r


with open("input.txt") as file:
    i = file.read()
    print(f"Part one: {part_one(i)}")
    print(f"Part two: {part_two(i)}")
