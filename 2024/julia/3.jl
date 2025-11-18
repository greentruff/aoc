input = read("input/3", String)

part1 = sum(
    r -> parse(Int, r[1]) * parse(Int, r[2]),
    eachmatch(r"mul\((\d{1,3}),(\d{1,3})\)", input)
)

@views function calc_part2(input::String)::Integer
    result = 0

    enabled = true
    index = 1
    while index < length(input)
        if enabled
            r = match(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)", input[index:end])
            if r == nothing
                break
            elseif r.match == "don't()"
                enabled = false
            else
                result += parse(Int, r[1]) * parse(Int, r[2])
            end
            index += r.offset + length(r.match) - 1
        else
            r = match(r"do\(\)", input[index:end])
            if r == nothing
                break
            else
                enabled = true
            end
            index += r.offset + length(r.match) - 1
        end
    end
    result
end
part2 = calc_part2(input)

println("Part 1: ", part1)
println("Part 2: ", part2)