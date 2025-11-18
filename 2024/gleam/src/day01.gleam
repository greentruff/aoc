import aoc
import gleam/int
import gleam/list
import gleam/result
import gleam/string

fn parse(input: String) -> Result(#(List(Int), List(Int)), Nil) {
  let data =
    input
    |> string.trim_end
    |> string.split("\n")
    |> list.map(string.split_once(_, "   "))
    |> result.all
    |> result.map(list.unzip)

  use #(left, right) <- result.try(data)

  let left = left |> list.map(int.parse) |> result.all
  let right = right |> list.map(int.parse) |> result.all

  case left, right {
    Ok(l), Ok(r) -> Ok(#(l, r))
    _, _ -> Error(Nil)
  }
}

fn part1(left: List(Int), right: List(Int)) -> Int {
  let left = list.sort(left, by: int.compare)
  let right = list.sort(right, by: int.compare)

  list.zip(left, right)
  |> list.map(fn(pair) {
    let #(left, right) = pair
    int.absolute_value(left - right)
  })
  |> int.sum
}

fn part2(left: List(Int), right: List(Int)) -> Int {
  list.fold(left, 0, fn(acc, l) {
    let right_occurences = list.count(right, fn(x) { x == l })
    acc + l * right_occurences
  })
}

pub fn solve(input) -> Result(aoc.Solution, Nil) {
  use data <- result.map(parse(input))

  let part1 = part1(data.0, data.1) |> int.to_string
  let part2 = part2(data.0, data.1) |> int.to_string

  aoc.Solution(part1:, part2:)
}
