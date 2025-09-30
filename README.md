# hamm

Compute the Hamming distance between s and t (_input file_: s and t separated by newline).
- Solution for Rosalind's [Counting Point Mutations](https://rosalind.info/problems/hamm/).
- Overcoded to make sure no bad input is entered.

Uses clap.rs.

# Usage

```
hamm <INPUT_FILE>
```

- INPUT_FILE must have s and t in first and second lines, respectively.

## Examples

### Basic Input
```
$ hamm test.txt
481
```

### Redirect output to file
Please note this overwrites a file if pre-existing.
```
$ hamm test.txt > test_output.txt
```

# Updates and Comments

## [2025-09-29]
- as_bytes() > chars()
  + UTF-8 characters, i.e. characters like in "CGAT", all take one byte.
  + This means we can make a "faster" check (I think!)
- I definitely could've done this in <40 lines of code (instead of 73)
if I didn't care about error proofing.
