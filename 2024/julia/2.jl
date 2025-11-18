input = readlines("input/2")
data = map(input) do line
    parse.(Int, split(line))
end

function issafe(line)
    @views diffs = line[begin+1:end] .- line[begin:end-1]
    continuous = all(>(0), diffs) || all(<(0), diffs)
    span = all(x -> 1 ≤ x ≤ 3, abs.(diffs))

    continuous && span
end

function issafe_dampened(line)
    issafe(line) || any(eachindex(line)) do idx
        dampened = deleteat!(copy(line), idx)
        issafe(dampened)
    end
end

part1 = count(issafe, data)
part2 = count(issafe_dampened, data)

println("Part 1: ", part1)
println("Part 2: ", part2)