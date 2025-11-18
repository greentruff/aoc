import kotlin.math.abs

fun main() {
    fun parseLists(input: List<String>): Pair<MutableList<Int>, MutableList<Int>> {
        val left = mutableListOf<Int>()
        val right = mutableListOf<Int>()
        for (line in input) {
            val parts = line.split("   ")
            left.add(parts[0].toInt())
            right.add(parts[1].toInt())
        }
        return Pair(left, right)
    }

    /*
        Sum of distance between lists
         */
    fun part1(input: List<String>): Int {
        val (left, right) = parseLists(input)

        left.sort()
        right.sort()

        var output = 0
        for (i in 0 until left.size) {
            val difference = abs(left[i] - right[i])
            output += difference
        }

        return output
    }

    /**
     * Sum of each number in left list * number of times it appears in right
     */
    fun part2(input: List<String>): Int {
        val (left, right) = parseLists(input)

        val rightOccurrences = mutableMapOf<Int, Int>()

        for (n in right) {
            rightOccurrences[n] = rightOccurrences.getOrDefault(n, 0).plus(1)
        }

        return left.sumOf { n -> n * rightOccurrences.getOrDefault(n, 0) }
    }

    val testInput = readInput("aoc1_test")
    check(part1(testInput) == 11)

    // Read the input from the `src/Day01.txt` file.
    val input = readInput("aoc1_input")
    println("Part 1:")
    part1(input).println()

    check(part2(testInput) == 31)
    println("Part 2:")
    part2(input).println()
}
