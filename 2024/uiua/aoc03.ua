Part₁ ← /+/×⋕ ↘₁⍉ regex "mul\\((\\d{1,3}),(\\d{1,3})\\)"
Part₂ ← Part₁ ▽⬚1\(↧⊙¬↥⊙°⊟)⍉⊟ ⊸∩⌕"do()","don't()"

┌─╴Tests
  Example ← $ xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))

  ⍤⟜≍: 161 Part₁ Example
  ⍤⟜≍: 48 Part₂ Example
└─╴

Input ← &fras "inputs/03.txt"

&p $"Part1: _" Part₁ Input # 192767529
&p $"Part2: _" Part₂ Input # 104083373
