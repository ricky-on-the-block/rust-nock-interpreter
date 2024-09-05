# Nock 4K Interpreter

A Rust implementation of the Nock virtual machine for the Nock 4K Definition

## Overview

This project implements a Nock interpreter in Rust. Nock is a minimalist, Turing-complete combinator calculus designed as the basis for the Urbit platform. Our interpreter:

- Uses a pure functional implementation in Rust
- Utilizes `Rc<>` for efficient memory management
- Uses simple recursion for clear, intuitive implementations
- Is developed using Behavior-Driven Development (BDD) based on the Nock specification

## Usage

To use the Nock interpreter, add it to your `Cargo.toml`:

```toml
[dependencies]
nock-interpreter = "0.2.0"
```

Then, in your Rust code:

```rs
fn main() {
    let input = noun![1 [2 3]];
    let result = nock(input);
    println!("Result: {:?}", result);
}
```

## Development
This project is developed using Test-Driven Development. To run the tests:
```rs
cargo test
```

## Benchmarking
To benchmark the implementation, run:
```rs
cargo bench
```

## Version 0.1.0 -> 0.2.0 Breaking Change
The interpreter has been re-implemented using `Rc<>` instead of `Box<>` to avoid the deep cloning performance hit. This change resulted in a significant speedup of 12.8x.

### Implementation Details
- Nouns are no longer edited in place for efficiency.
- The Interpreter is a pure functional implementation in Rust.
- It still uses recursion and continues to use BDD/TDD for development.

## Future Work
- Ensure all recursion is tail-call optimized for better performance and to handle larger Nock programs
- Implement a parser to allow Nock input in string format, making it easier to write and test Nock programs
- Add console logs to display the input and output of the interpreter, aiding in debugging and understanding Nock execution
- Consider multi-threaded implementation using `Arc<>` over `Rc<>`
- Consider caching certain aspects of the Noun Binary Tree

## Nock 4K Definition

Nock is defined by the following rules:
- An atom is a natural number.
- A noun is an atom or a cell (pair) of nouns.

Reduce by the first matching pattern; variables match any noun.

```text
nock(a)             *a
[a b c]             [a [b c]]

?[a b]              0
?a                  1
+[a b]              +[a b]
+a                  1 + a
=[a a]              0
=[a b]              1

/[1 a]              a
/[2 a b]            a
/[3 a b]            b
/[(a + a) b]        /[2 /[a b]]
/[(a + a + 1) b]    /[3 /[a b]]
/a                  /a

#[1 a b]            a
#[(a + a) b c]      #[a [b /[(a + a + 1) c]] c]
#[(a + a + 1) b c]  #[a [/[(a + a) c] b] c]
#a                  #a

*[a [b c] d]        [*[a b c] *[a d]]

*[a 0 b]            /[b a]
*[a 1 b]            b
*[a 2 b c]          *[*[a b] *[a c]]
*[a 3 b]            ?*[a b]
*[a 4 b]            +*[a b]
*[a 5 b c]          =[*[a b] *[a c]]

*[a 6 b c d]        *[a *[[c d] 0 *[[2 3] 0 *[a 4 4 b]]]]
*[a 7 b c]          *[*[a b] c]
*[a 8 b c]          *[[*[a b] a] c]
*[a 9 b c]          *[*[a c] 2 [0 1] 0 b]
*[a 10 [b c] d]     #[b *[a c] *[a d]]

*[a 11 [b c] d]     *[[*[a c] *[a d]] 0 3]
*[a 11 b c]         *[a c]

*a                  *a
```

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
