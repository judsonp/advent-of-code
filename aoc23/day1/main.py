from typing import Dict, List, Optional

digit_representations: Dict[str, int] = {
    "1": 1, "2": 2, "3": 3, "4": 4, "5": 5, "6": 6, "7": 7, "8": 8, "9": 9,
    "one": 1, "two": 2, "three": 3, "four": 4, "five": 5,
    "six": 6, "seven": 7, "eight": 8, "nine": 9,
}


def digit(s: str) -> Optional[int]:
    return next((d for t, d in digit_representations.items() if s.endswith(t)), None)


def digits(s: str) -> List[int]:
    return list(
        filter(lambda x: x is not None,
               map(lambda substr: digit(substr),
                   (s[0:end] for end in range(len(s))))))


def value(s: str) -> int:
    d = digits(s)
    return d[0] * 10 + d[-1]


def run():
    with open("input.txt") as f:
        print(sum([value(s) for s in f.readlines()]))


run()
