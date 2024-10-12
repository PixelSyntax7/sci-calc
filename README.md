# Scientific Calculator

## Grammar
The grammar is written in [extended backus naur form](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form).
```
Expr = Term
     | ["+" | "-"] Expr
     | "(" Expr ")"

Term = Factor
     | Term ("+" | "-") Factor

Factor = Primary
       | Factor ("*" | "/" | "%") Primary
       | Primary "^" Factor

Primary = Int
        | Float
        | Func "(" ArgList ")"
        | Const

ArgList = Expr ["," Expr]*

Int = DIGITS
Float = DIGITS "." DIGITS [("e" | "E") [("+" | "-")] DIGITS]
Func = CHARS
Const = CHARS

CHAR = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"
CHARS = CHAR+
DIGIT = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
DIGITS = DIGIT+
```
