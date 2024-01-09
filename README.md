# List Comprehender
A list comprehension interpreter with haskell-like / "set builder"-like syntax
## Syntax
`{ <element operation> | {element filter} in [<range min> .. <range max>] }`
- Element operation: a calculation involving an element variable  
Example: `x^2`,
- Element filter: a boolean statement that will be true for all returned elements in the given range  
Example: `x % 5 = 0`
- Range mix/max: the (inclusive) minimum and maximum of the range to pull elements from  
Example: `[0..100]`  

Complete example: `{ x^2 | x % 5 = 0 in [0..100] }`  
This will produce a list of every multiple of 5 from 0 to 100, squared
### Operators
- `+`: Add
- `-`: Subtract
- `*`: Multiply
- `/`: Divide
- `^`: Exponentation
- `%`: Modulus
- `//`: Integer Division
- `=`: Equality
- `!=`: Inequality
- `>`/`>=`: Greater than / Greater than or equal to
- `<`/`<=`: Less than / Less than or equal to
## Future Features
- [ ] Multiple element ranges
- [ ] Multiple element filters for a range (using logical operators)
- [ ] `_` to indicate unused element values in range declaration