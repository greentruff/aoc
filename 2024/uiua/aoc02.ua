Parse ← ⊜(□⊜⋕≠@\s.)≠@\n.

Check ← /↧↧⊃>₀≤₃ ⍥¯<0⊸⊢ ⧈(/-) 2
Part₁ ← /+≡◇Check

Part₂ ← /+≡◇(/↥≡(Check⍜↻↘₁)⊙¤°⊏)
┌─╴Tests
  Example ← $ 7 6 4 2 1
            $ 1 2 7 8 9
            $ 9 7 6 2 1
            $ 1 3 2 4 5
            $ 8 6 4 4 1
            $ 1 3 6 7 9
            $ 
  ⍤⟜≍: 2 Part₁ Parse Example
  ⍤⟜≍: 4 Part₂ Parse Example
└─╴

Input ← Parse &fras "inputs/02.txt"

&p $"Part1: _" Part₁ Input # 624
&p $"Part2: _" Part₂ Input # 658
