Parse ← ⊜∘≠@\n.

Rotate ← ⍉≡(↻⊙⊂:" ")
Diag   ← Rotate °⊏
ADiag  ← Rotate ¯°⊏

Part₁ ← /+ ≡(/+≡/+◇⌕"XMAS") {⊃(∘|≡⇌|⍉|≡⇌⍉|Diag|ADiag|Diag⇌|ADiag⇌)}
Part₂ ← /+ ♭⧈(∈["MMASS" "MSAMS" "SMASM" "SSAMM"] ⊡[0_0 0_2 1_1 2_0 2_2]) 3_3

┌─╴Tests
  Example ← $ MMMSXXMASM
            $ MSAMXMSMSA
            $ AMXSXMAAMM
            $ MSAMASMSMX
            $ XMASAMXAMM
            $ XXAMMXXAMA
            $ SMSMSASXSS
            $ SAXAMASAAA
            $ MAMMMXMMMM
            $ MXMXAXMASX
  ⍤⤙≍ 1 /+ ♭ ≡(⌕"XMAS") Diag Parse Example
  ⍤⤙≍ 18 Part₁ Parse Example
  ⍤⤙≍ 9 Part₂ Parse Example
└─╴

Input ← Parse &fras "inputs/04.txt"

&p $"Part1: _" Part₁ Input # 2507
&p $"Part2: _" Part₂ Input # 1969
