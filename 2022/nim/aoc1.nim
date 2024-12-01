# https://adventofcode.com/2022/day/1

import algorithm
import strutils

iterator elf_iter() : int =
    let f = open("aoc1.input")
    defer: f.close()

    var line: string
    var elf_load = 0
    while f.read_line(line):
        if len(line) > 0:
            let load =  parseInt(line)
            elf_load += load
        else:
            yield elf_load
            elf_load = 0
    if elf_load > 0:
        yield elf_load

var loads = newSeq[int]()
for load in elf_iter():
    loads.add(load)
sort(loads)

echo "Max: ", loads[len(loads)-1]
echo "Top 3: ", loads[len(loads)-1] + loads[len(loads)-2] + loads[len(loads)-3]
