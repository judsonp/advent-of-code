from itertools import combinations
from typing import NamedTuple, List, Optional


MIN_XY = 200000000000000
MAX_XY = 400000000000000


class Vector(NamedTuple):
    x: float
    y: float
    z: float

    def __str__(self):
        return f"({self.x}, {self.y}, {self.z})"

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y, self.z + other.z)


class Hailstone(NamedTuple):
    p: Vector
    v: Vector

    def __str__(self):
        return f"[p: {self.p}, v: {self.v}]"


# a, b are two points on line 1; c, d are two points on line 2
# the z coordinate is ignored and not filled in
# returns None if the lines do not intersect
def intersect(a: Vector, b: Vector, c: Vector, d: Vector) -> Optional[Vector]:
    a1 = b.y - a.y
    b1 = a.x - b.x
    c1 = a1 * a.x + b1 * a.y
    a2 = d.y - c.y
    b2 = c.x - d.x
    c2 = a2 * c.x + b2 * c.y

    d = a1 * b2 - a2 * b1

    if d == 0:
        return None
    else:
        x = (b2 * c1 - b1 * c2) / d
        y = (a1 * c2 - a2 * c1) / d
        return Vector(x, y, 0)


def parse() -> List[Hailstone]:
    hailstones = []
    with open("../inputs/day24.txt") as f:
        for line in f:
            line = line.strip()
            [p, v] = line.split(' @ ')
            [px, py, pz] = map(lambda x: float(x), p.split(', '))
            [vx, vy, vz] = map(lambda x: float(x), v.split(', '))
            hailstones.append(Hailstone(Vector(px, py, pz), Vector(vx, vy, vz)))
    return hailstones


def part_one(hailstones: List[Hailstone]) -> int:
    count = 0
    for (h1, h2) in combinations(hailstones, 2):
        a = h1.p
        b = h1.p + h1.v
        c = h2.p
        d = h2.p + h2.v
        i = intersect(a, b, c, d)
        if i is not None:
            t1 = (i.x - h1.p.x) / h1.v.x
            t2 = (i.x - h2.p.x) / h2.v.x
            if t1 >= 0 and t2 >= 0 and MIN_XY <= i.x <= MAX_XY and MIN_XY <= i.y <= MAX_XY:
                count += 1
    return count


def run():
    hailstones = parse()
    print(part_one(hailstones))


run()
