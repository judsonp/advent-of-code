def part_one(lines):
    cards = [map(lambda x: x.strip().split(), line.split(":")[1].split("|")) for line in lines]
    scores = map(lambda x: 2 ** (x - 1) if x > 0 else 0,
                 [sum(map(lambda x: 1, (filter(lambda x: x in winning, have)))) for winning, have in cards])
    return sum(scores)


def part_two(lines):
    cards = [map(lambda x: x.strip().split(), line.split(":")[1].split("|")) for line in lines]
    scores = [sum(map(lambda x: 1, (filter(lambda x: x in winning, have)))) for winning, have in cards]
    counts = [1] * len(scores)
    for i in range(0, len(counts)):
        for j in range(0, scores[i]):
            counts[i+j+1] += counts[i]
    return sum(counts)


with open("input.txt") as file:
    input = file.readlines()
    print(f"Part one: {part_one(input)}")
    print(f"Part two: {part_two(input)}")
