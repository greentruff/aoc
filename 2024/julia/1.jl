input = readlines("input/1")

data = reduce(hcat, parse.(Int, split(line)) for line in input)
left, right = sort.(eachrow(data))

part1 = abs.(left .- right) |> sum
part2 = sum(left) do x
    x * count(==(x), right)
end

println("Part 1: ", part1)
println("Part 2: ", part2)