Parse ← :⍉◇⊓(⊜(⊜⋕≠@|.)≠@\n.|⊜(□⊜⋕≠@,.)≠@\n.) °⊟⊜□ ¬⦷"\n\n".

# Sorts the pages by repeatedly taking pages that have no rule for next
Order ← |1 (
  ⨬([]
  | ⇌⊢⍉
  | ⊂⊃(
      Order⍜(⊙⍉)▽
    | ◴⊢⍜(⊙⍉)▽¬
    ) ∈:⊸°⊟
  ) ↧2⧻⍉.
)

TopoSort ← (
  ⍜(⊙⍉)▽ ◇⊃(∈∘⊙⊣|⋅∘|∘)
  ▽◡⍜:∈ ⇌Order
)
Middle ← ⊡⌊÷2⧻.

Part₁ ← (
  ⊙¤
  ≡(
    TopoSort
    ⊸⊗
    Middle ×/↧⧈/>2
  )
  /+
)

Part₂ ← (
  ⊙¤
  ≡(
    TopoSort
    ⟜∩⍏
    ×⊙(/↥≠)
    Middle
  )
  /+
)
┌─╴Tests
  Example ← $ 47|53
            $ 97|13
            $ 97|61
            $ 97|47
            $ 75|29
            $ 61|13
            $ 75|53
            $ 29|13
            $ 97|29
            $ 53|29
            $ 61|53
            $ 97|53
            $ 61|29
            $ 47|13
            $ 75|47
            $ 97|75
            $ 47|61
            $ 75|61
            $ 47|29
            $ 75|13
            $ 53|13
            $ 
            $ 75,47,61,53,29
            $ 97,61,53,29,13
            $ 75,29,13
            $ 75,97,47,61,53
            $ 61,13,29
            $ 97,13,75,29,47
  
  ⍤⤙≍ 143 Part₁ Parse Example
  ⍤⤙≍ 123 Part₂ Parse Example
└─╴

Input ← Parse &fras "inputs/05.txt"

# &p $"Part1: _" Part₁ Input # 7024
# &p $"Part2: _" Part₂ Input # 4151
