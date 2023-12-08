import math
from enum import Enum

class ordering(Enum):
    normal = "AKQJT98765432"
    jokers = "AKQT98765432J"

def entropy(hcards):
    return -sum(map(
        lambda p: p * math.log(p),
        [
            hcards.count(c) / len(hcards)
            for c in set(hcards)
        ],
    ))

def entropy_with_jokers(hcards):
    try:
        top = sorted(
            hcards.replace("J", ""),
            key=lambda c: hcards.count(c),
        )[-1]
        return entropy(hcards.replace("J", top))
    except IndexError:
        return entropy(hcards)

with open("input.txt") as src:
    hands = [
        (h, int(b))
        for h, b in map(str.split, src)
    ]

print("part two answer:", sum([
    (len(hands) - i) * bid
    for i, (__, bid) in enumerate(sorted(
        hands,
        key=lambda h: (
            entropy_with_jokers(h[0]),
            *map(ordering.jokers.value.index, h[0]),
        )
    ))
]))

