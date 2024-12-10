Parse ← ⊜(⊜⋕≠@\s.)≠@\n.
Part₁ ← /+ ⌵/- ≡⍆ ⍉

Part₂ ← /+ ×⟜≡(/+⌕) ⊙¤°⊟ ⍉

┌─╴Tests
  Example ← $ 3   4
            $ 4   3
            $ 2   5
            $ 1   3
            $ 3   9
            $ 3   3
            $ 
  ⍤⤙≍ 11 Part₁ Parse Example
  ⍤⤙≍ 31 Part₂ Parse Example
└─╴

Input ← Parse &fras "inputs/01.txt"

&p $"Part1: _" Part₁ Input # 1938424
&p $"Part2: _" Part₂ Input # 22014209
