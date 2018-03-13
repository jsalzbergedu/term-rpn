# term-rpn

## What it is

A simple [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) calculator for the terminal.

## How to use it

To push a number to the stack, call `rpn` and enter numbers as parameters:

```bash
rpn 3 4
# Prints 3 4
```

This program has four operators, add, subtract, multiply, and divide. You can enter them with the letters
'a' (for add), 's' (for subtract), 'm' (for multiply), and 'd' (for divide). Example:

```bash
rpn 3 4 a
# prints 7
```

### Multiple entries
Since the calculator is stack based, you can enter multiple numbers like so:

(3 + 4) / 10:
```bash
rpn 3 4 a 10 d
```

((3 + 4) / 10) * (10 / 2):
```bash
rpn 3 4 a 10 d 10 2 d m
```

### Floating points
Rpn uses integers, unless you type floating point numbers:
```bash
rpn 1 2 d
# prints 0
rpn 1.0 2 d
# prints .5
```

## Installing

Clone this directory, and run `cargo-install`.
