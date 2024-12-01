import fileinput
import os
import re
import typing
from pprint import pprint as pp

def debug(value: typing.Any):
    if os.environ.get('DEBUG', None):
        pp(value)

class SchematicValue(typing.NamedTuple):
    value: str
    row: int
    column: int

    def col_start(self):
        return self.column
    
    def col_end(self):
        return self.column + len(self.value) - 1

    def filter_adjacent(self, values: list[typing.Self]) -> list[typing.Self]:
        adjacent = []
        for value in values:
            adj_row = value.row >= self.row-1 and value.row <= self.row+1
            adj_col = (value.col_end() >= self.col_start()-1 
                       and value.col_start() <= self.col_end()+1)
            if adj_row and adj_col:
                adjacent.append(value)
        return adjacent

value_regex = re.compile(r"\d+|[^.0-9]")
row = 0

numbers: list[SchematicValue] = []
symbols: list[SchematicValue] = []
for line in fileinput.input():
    for match in value_regex.finditer(line.strip()):
        value = SchematicValue(value=match.group(0), row=row, column=match.start())
        if value.value.isdigit():
            numbers.append(value)
        else:
            symbols.append(value)
    row += 1


part_numbers: list[SchematicValue] = []
for number in numbers:
    if number.filter_adjacent(symbols):
        # debug("Y " + str(number) + " - " + str(candidate_symbols))
        debug("Y " + str(number))
        part_numbers.append(number)
    else:
        # debug("N " + str(number) + " - " + str(candidate_symbols))
        debug("N " + str(number))

gear_ratios: list[int] = []
for symbol in symbols:
    if symbol.value == '*':
        adj = symbol.filter_adjacent(numbers)
        if len(adj) == 2:
            gear_ratios.append(int(adj[0].value)*int(adj[1].value))

# debug(str(part_numbers))
part_values = list(int(p.value) for p in part_numbers)
debug(str(part_values))
print(f"Part 1: {sum(part_values)}")
print(f"Part 2: {sum(gear_ratios)}")