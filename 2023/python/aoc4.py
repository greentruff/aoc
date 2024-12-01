import fileinput
import os
from pprint import pprint as pp
import typing
import collections


def debug(value: typing.Any):
    if os.environ.get('DEBUG', None):
        pp(value)

points = 0
card_counts: dict[int, int] = collections.defaultdict(int)

for line in fileinput.input():
    # Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    first_sep = line.find(':')
    id = int(line[5:first_sep])
    winning_numbers_raw, owned_numbers_raw = line[first_sep:].split('|', maxsplit=1)

    winning_numbers = set(winning_numbers_raw.split())
    owned_numbers = set(owned_numbers_raw.split())

    owned_winning_numbers = winning_numbers.intersection(owned_numbers)

    # part 1
    if len(owned_winning_numbers) > 0:
        points += pow(2, len(owned_winning_numbers)-1)
    
    # part 2
    card_counts[id] += 1
    if len(owned_winning_numbers) > 0:
        for virtual_id in range(len(owned_winning_numbers)):
            card_counts[id + 1 + virtual_id] += card_counts[id]

    
print(f"Part 1: {points}")

debug(card_counts)
print(f"Part 2: {sum(card_counts.values())}")