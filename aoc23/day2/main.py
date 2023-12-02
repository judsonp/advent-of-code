import collections
import math
import re


def power(line: str) -> int:
    cubes = collections.defaultdict(int)
    for num, col in re.findall(r'(\d+) (\w+)', line):
        cubes[col] = max(cubes[col], int(num))
    return math.prod(cubes.values())


with open("input.txt") as file:
    print(sum((power(line) for line in file.readlines())))