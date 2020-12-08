# AdventOfCode-2020

## Challenge 1

Solution: get all possible combinations and validate them (n k). Combinations are calculated using the itertools library.

#### Part1:

2 entries, sum == 2020
multiply the 2 entries == answer

#### Part2:

3 entries, sum == 2020
multiply the 3 entries == answer

## Challenge 2

Parse text and validate password

#### Part 1:
Simple char count

#### Part 2:
Strings cannot be index in rust, but since we are sure it only contains ASCII characters we can access the raw bytes and cast them to chars.

## Challenge 3

#### Part 1:
Simple vector traversal with checking indices for exceeding max length

#### Part 2:
Make part 1 a function with input for the slope and multiply the results.

## Challenge 8

#### Part 2:
For this part the instruction that causes the loop needs to be found and rectified.
A brute force approach would be to change every NOP / JMP one by one and validate if the loop computes.

A more efficient way would be to run the loop and detect the looping seqeuence. If we take the example:

| Index| Instruction | Value  |
| - |:---:| --:|
| 0 | nop | +0 |
| 1 | acc | +1 |
| 2 | jmp | +4 |
| 3 | acc | +3 |
| 3 | jmp | -3 |
| 4 | acc | -99 |
| 5 | acc | +1 |
| 6 | jmp | -4 |
| 7 | acc | +6 |

I we print the indexes in the order we execute them:

[ 0, 1, 2, 6, 7, 3, 4, 1, 2, 6, ... ]

We can detect that there is a pattern:

[ 1, 2, 6, 7, 3, 4 ]

If we only leave the NOP/JMP instructions we are left with the following set of instructions:

[ 2, 7, 4]

We can now change these 1 by 1 to vaiidate which one fixes the loop