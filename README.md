# A Compiler for a toy functional language
This project is a work in progress and it fulfills my goal of writing a compiler for a functional language.
## The compiler
The compiler (and the VM in the future) are completely written in Rust
## The language
The language is a simple functional language with the following basic features
- Tagged union datatypes
- pattern matching
- currying
- capturing enironment (closures)

The following things are missing
- mutual recursive types and functions
- floating point data type