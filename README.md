# ceen: a functional language

ceen (_si:n_) is an ML-like strongly-typed functional language. Currently it's just a toy language for me to practice compiler concepts. The REPL for ceen is cerebral.

## Current Features

### Type system

* integer, string and unit literals
* tuples
* Recursive tagged unions
* higher-order function type

Hindly-Milner fully inferred types.

### Functions

Functions are first-class values and support nesting and capturing values from the enclosing scope.

### Pattern Matching

Pattern matching happens at the function arguments level and at the top-level declarations level. Function patterns are checked to be exhaustive and non-redundant. Patterns can optionally have type annotations

## Syntax

Because the language is not meant to be usable (for now), many of the conveniant structures are not implemented. No assignment, if expressions or scoped-binding. The last two can be implemented through functions and pattern matching.

### Type declaration

```
type <name> <type-param-names*> =
    | <variant-name> <type>
    | ...
```

### Top-level declarations

```
let <pat> = <expr>
let <pat> = <expr>
```

### Functions

Function values are written as
```
{ 
    <pat1> => <exp1>, 
    <pat2> => <exp2> ... 
}
```
Recursive function have to be declared with keyword rec instead of let
```
rec fib = { 0 => 1, 1 => 1, n => fib (n-1) + fib(n-2) }
```

## Library functions

Currently, two functions are hardcoded into the repl: `print: string -> ()` and `i2str: int -> str`

## Examples:

``` Algebraic Types
type Tree t =
    | Node (Tree t, t, Tree t)
    | Empty ()

let t = Node (Empty(), 1, Node (Empty(), 2 Empty()))
```

``` Pattern Matching
rec mapTree = {
    (Empty()) f        => Empty(),
    (Node (l, x, r)) f => Node (mapTree l f, f x, mapTree r f),
}
```

``` Closures
// CPS map
rec mapTree = {
    (Empty()) f k        => Empty(),
    (Node (l, x, r)) f k => mapTree l f 
        {l' => mapTree r f {r' => Node(l', f x, r')}},
}
```
