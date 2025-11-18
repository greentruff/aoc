input = reduce(vcat, permutedims(collect(line)) for line in eachline("input/4"))

function countxmas(input)
    DIRECTIONS = map(CartesianIndex, [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1)
    ])

    count = 0
    for cur in eachindex(IndexCartesian(), input)
        if input[cur] == 'X'
            for dir in DIRECTIONS
                midx = cur + dir
                aidx = midx + dir
                sidx = aidx + dir

                if checkbounds(Bool, input, sidx) && input[midx] == 'M' && input[aidx] == 'A' && input[sidx] == 'S'
                    count += 1
                end
            end
        end
    end
    count
end

function countcrossmas(input)
    count = 0
    for cur in eachindex(IndexCartesian(), input)
        if input[cur] == 'A'
            corners = cur .+ map(CartesianIndex, [(-1, -1), (1, 1), (-1, 1), (1, -1)])

            if (checkbounds(Bool, input, corners)
                && sort([input[i] for i in corners[1:2]]) == ['M', 'S']
                && sort([input[i] for i in corners[3:4]]) == ['M', 'S'])
                count += 1
            end
        end
    end
    count
end

part1 = countxmas(input)
part2 = countcrossmas(input)

println("Part 1:", part1)
println("Part 2:", part2)