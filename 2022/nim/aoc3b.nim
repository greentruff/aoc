import algorithm

func itemPriority(c: char): int =
    if int(c) >= int('a') and int(c) <= int('z'):
        result = int(c) - int('a') + 1
    else:
        result = int(c) - int('A') + 27

iterator rucksacks(): (seq[int],seq[int],seq[int]) =
    let f = open("aoc3.input")
    defer: f.close()

    var line: string
    while f.read_line(line):
        var first = newSeq[int]()
        for itemIdx in 0 .. (len(line) - 1):
            first.add(itemPriority(line[itemIdx]))
        
        discard f.read_line(line)
        var second = newSeq[int]()
        for itemIdx in 0 .. (len(line) - 1):
            second.add(itemPriority(line[itemIdx]))

        discard f.read_line(line)
        var third = newSeq[int]()
        for itemIdx in 0 .. (len(line) - 1):
            third.add(itemPriority(line[itemIdx]))

        yield (sorted(first), sorted(second), sorted(third))

var totalPrio = 0
for sac in rucksacks():
    let first = sac[0]
    let second = sac[1]
    let third = sac[2]

    var firstIdx = 0
    var secondIdx = 0
    var thirdIdx = 0
    
    var prio = 0
    while prio == 0:
        let firstPrio = first[firstIdx]
        let secondPrio = second[secondIdx]
        let thirdPrio = third[thirdIdx]
        if firstPrio == secondPrio and secondPrio == thirdPrio:
            prio = firstPrio
        elif firstPrio < secondPrio:
            if firstPrio < thirdPrio and firstIdx < len(first)-1:
                firstIdx += 1
            elif thirdIdx < len(third)-1:
                thirdIdx += 1
            else:
                secondIdx += 1
        else:
            if secondPrio < thirdPrio and secondIdx < len(second)-1:
                secondIdx += 1
            elif thirdIdx < len(third)-1:
                thirdIdx += 1
            else:
                firstIdx += 1
    totalPrio += prio

echo totalPrio