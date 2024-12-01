import strformat

func pythonMod(n, M: int): int = ((n mod M) + M) mod M

iterator rps(): (int,int) =
    let f = open("aoc2.input")
    defer: f.close()

    var line: string
    while f.read_line(line):
        let them = int(line[0]) - int('A')
        # let me = int(line[2]) - int('X')
        let roundResult = int(line[2]) - int('X')
        let me = pythonMod(them + roundResult - 1, 3)
        yield (them, me)

var roundCount = 0
var score = 0
let RPS = @['R', 'P', 'S']

for round in rps():
    roundCount += 1

    let roundScore = pythonMod((round[1] - round[0] + 1), 3) * 3 + round[1] + 1
    score += roundScore
    echo fmt"{roundCount}: {RPS[round[0]]}-{RPS[round[1]]} -- {roundScore} -- {score}"