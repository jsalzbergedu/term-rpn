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

## Installing

Clone this directory, and run `cargo-install`.
