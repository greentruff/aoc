import day01
import day02
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import simplifile

const days = [
  day01.solve,
  day02.solve,
]

pub fn main() -> Nil {
  list.index_map(days, fn(solve, day) {
    let day = int.to_string(day + 1)
    io.println("==== Day " <> day <> " ====")
    let _ = case simplifile.read("input/" <> day) {
      Ok(input) -> {
        use solution <- result.map(solve(input))

        io.println("Part 1: " <> solution.part1)
        io.println("Part 2: " <> solution.part2)
      }
      Error(e) -> {
        io.print_error(simplifile.describe_error(e))
        Ok(Nil)
      }
    }
    io.println("")
    Ok(Nil)
  })
  Nil
}
