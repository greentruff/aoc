import strutils

iterator assignments() : ((int,int), (int,int)) =
    let f = open("aoc4.input")
    defer: f.close()

    var line: string
    while f.read_line(line):
        let ranges = line.split(',')
        func toRange(range: string): (int,int) =
            let splitRange = range.split('-')
            return (parseInt(splitRange[0]), parseInt(splitRange[1]))
        yield (toRange(ranges[0]), toRange(ranges[1]))

# func contains(tuple1: (int,int), tuple2: (int,int)): bool =
#     return tuple2[0] >= tuple1[0] and tuple2[1] <= tuple1[1]

func overlaps(tuple1: (int,int), tuple2: (int,int)): bool =
    return (tuple1[0] >= tuple2[0] and tuple1[0] <= tuple2[1]) or (tuple1[1] >= tuple2[0] and tuple1[1] <= tuple2[1])

var count = 0
for assignment in assignments():
    let 
        tuple1 = assignment[0]
        tuple2 = assignment[1]
    if tuple1.overlaps(tuple2) or tuple2.overlaps(tuple1):
        echo assignment
        count += 1

echo count
