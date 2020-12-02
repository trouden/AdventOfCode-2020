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