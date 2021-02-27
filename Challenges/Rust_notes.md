# Quick notes about Rust

Arithmetics:
numeric types have to be explicit (_i.e_ you cannot do u8 + u32 directly. Instead: **u8 as u32** + u32)
**as** is a for type conversions

variable and function names are always in **snake_case**
Constant names are always in **SCREAMING_SNAKE_CASE**

arrays: fixed size, same type 1D data structure. It is defined: `[type; length]` 
Arrays cannot be printed just calling the variable `println("{}", array)` they have their own format `println("{:?}", array)`
Negative indexes do not work!! 

There are tuples too! empty tuples are known as a _unit_ and are represented as `()`
Tuples can have elements of different types

`if`'s are as you would expect.

From cycles:
- **loop** infinite cycle (loop can break and return a value)
- **while** conditional cycle
- **for** finite cycle, works with any iterable

- **enum** creates a enumeration, a limited set of defined values a variable can assume

### Ownership!!

The ownership rules are:

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

#### References

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.


- let: used to define variables
- use: to import crates
- :: is used to connect libraries (pandas::dataframe)
- fn: define functions
- struct: Create classes or data structures
- match: is like case. Every case has to be covered always.
- impl: methods
- trait: methods shared (definitively we have to check these further). There are markers too
- 
