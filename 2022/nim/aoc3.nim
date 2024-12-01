import algorithm

func itemPriority(c: char): int =
    if int(c) >= int('a') and int(c) <= int('z'):
        result = int(c) - int('a') + 1
    else:
        result = int(c) - int('A') + 27

iterator rucksacks(): (seq[int],seq[int]) =
    let f = open("aoc3.input")
    defer: f.close()

    var line: string
    while f.read_line(line):
        let ruckSize = len(line) /% 2
        var first = newSeq[int]()
        var second = newSeq[int]()
        
        for itemIdx in 0 .. (ruckSize - 1):
            first.add(itemPriority(line[itemIdx]))
        for itemIdx in ruckSize .. (len(line) - 1):
            second.add(itemPriority(line[itemIdx]))

        yield (sorted(first), sorted(second))

var misplacedPrio = 0
for sac in rucksacks():
    let first = sac[0]
    let second = sac[1]
    var firstIdx = 0
    var secondIdx = 0
    
    var prio = 0
    while prio == 0:
        if first[firstIdx] == second[secondIdx]:
            prio = first[firstIdx]
        elif first[firstIdx] < second[secondIdx]:
            firstIdx += 1
        else:
            secondIdx += 1
    misplacedPrio += prio

echo misplacedPrio