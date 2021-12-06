# Solutions for [Advent of Code 2021](https://adventofcode.com/2021)

This year I wanted to learn Rust.

# Day 1 notes

Today I installed Rust, struggled a bit with a broken Visual Studio installation.

Today I learned:

- Structs and traits (though ultimately unused)
- Type basics (borrowing is still a mystery)
- println! (but didn't dig into macros)

## Task notes

A comparison of "sliding windows" is really a comparison of the numbers which are not in common between the two windows.
If measurements (1, 2, 3, 4) are parts of the measurements A and B as below, the difference between A and B is the difference between 1 and 4.

```
1 A
2 A B
3 A B
4   B
```

Rather than calculate or otherwise construct a sliding window, we just compare numbers with an offset.

# Day 2 notes

Today I learned:

- Reading input from files (and fixed up day 1 to do so)
- Pattern matching on vectors
- Derived traits
- Testing
- `pub` and `use`

... though I didn't get around to it until Dec 6th.

## Task notes

Mixing up absolute/relative movement seems to be a recurring theme in AoC. Nothing much to note about the task itself.
