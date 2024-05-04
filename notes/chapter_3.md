## Variables and mutability
Variables are immutable by default! Making variables mutable after a value is assigned to them is an actual choice this time around, by adding `mut` when declaring a variable. Variable type must remain the same though. _However..._

**Shadowing:** redefining a variable with `let` again within a certain scope is possible, and can be used to change the value **or even the type** of a variable for the entire time within that scope. This is only possible by explicitly using `let`, avoiding accidental re-assigns and still having the var be immutable otherwise.

Immutable variable =/= constant variable. `const` variables do not allow `mut` to be added, and demand type annotations. `const` must be written in CAPS_LOCK, and calculated at compile time.

## Data types
Rust is statically typed --> compiler must know all variable types at compile time. Can have the compiler try to infer it, but type annotations may be required sometimes. Different data types are listed in ch 3.2.

Scalar types: represent a single value, i.e. ints, floats, bools, and chars. For numbers you can use _ as a visual separator (no actual effect). Types can be suffixed and notations prefixed, e.g. `57u8` (unsigned 8-bit int) or `0xff` (hex value of `ff`).

Overflows causing a panic at runtime in debug, but wrap around on release builds. Special function exist to specifically handle this instead, check ch 3.2.

Char type uses single quotes '', string literal uses double quotes ""

Tuples are fixed-length, but can contain multiple types (55, 7.1, '?')
-> Accessing via periods: tuple.1 instead of tuple\[1\]
-> Type declaration ex.: `let x: (i32, f64, u8) = (500, 6.4, 1);`

Arrays are also fixed-length, but can only contain one type. Written in \[square brackets\].
-> Accessing via square brackets, as usual
-> Declaration ex.: `let a: [i32; 5] = [1, 2, 3, 4, 5];`

Vectors (std lib) are allowed to grow or shrink, but are covered in ch 8.

An expression without a return implicitly returns the _unit_, aka () (empty tuple)

## Functions

Basic syntax example:
`
fn functional(x: i32) {
    println!("The value of x is: {x}");
}
`

Note: you _must_ declare the type of each parameter!

Uniquely Rust: a function is made up of statements, and optionally ends in an expression:
- A statement is an instruction/does a thing, and does not return a value. For example, `let y = 6;` is a statement as it doesn't return anything. A function declaration is a statement too, it doesn't return anything either.
- An expression evaluates to a resultant value. Calling a function or a macro is an expression for example.
    - More uniquely, a scope block (created with curly brackets) is an expression that evaluates to a value.
    - Expressions do NOT include ending semicolons! If you do add a semicolon, it turns into a statement instead and will not return a value.

Function returns are something special. You must declare the type after an arrow (kinda like Python type annotations, but required). You can use `return`, esp in the body of a function, but at the end of a function the last expression can also be returned implicitly. This is a perfectly valid function for example (note the lack of a semicolon on the last line):

`
fn five() -> i32 {
    5
}
`

## Code comments

// Comment.

Multiline comments: just start each line with `//`.

For explanation on documentation comments: Chapter 14. That's a while away.

## Control Flow
_Ooh yeah. It's all coming together._

If statements: curly brackets for the body of the `true` & `false` arms, the if statement itself is not within brackets or anything. The content of the `if` must explicitly be/evaluate to a bool, there will be no implicit conversions (instead, have an error :) ). Opening brackets are on the same line as the statement, already showed this for fn's but also for other places:
`
fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else if else {
        println!("condition was false");
    }
}
`

You can also use if statements for assignments, kind of like how you can use lambda statements in Python:
`let number = if condition { 5 } else { 6 };`

## Repeating code: loops and stuff

The `loop` keyword is basically `while True` but it's even shorter to write in Rust. The `continue` and `break` keywords also exist, but `break` works differently than usual: it functions closer to a `return`, except it breaks out of a loop and returns something from the loop instead of returning in the way `return` usually does.

Loops can also be labeled, so that you can specify which loop a `break` or `continue` is supposed to break out of. _No more break chains with booleans recognizing higher up breaks!_ 

Also, `while` still exists too. Same with `for` loops, those are nothing special either. Pythonic syntax, with for example `for element in a {println!(a)}`.
