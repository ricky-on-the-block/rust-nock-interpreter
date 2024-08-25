/// # rust-nock-interpreter
///
/// A Rust implementation of the Nock 4K virtual machine.
///
/// ## Nock 4K Specification
///
/// Nock is a simple, Turing-complete combinator algebra. It operates on nouns, which are either
/// atoms (natural numbers) or cells (ordered pairs of nouns).
///
/// ### Basic Definitions
///
/// - A noun is an atom or a cell.
/// - An atom is a natural number.
/// - A cell is an ordered pair of nouns.
///
/// ### Reduction Rules
///
/// The interpreter reduces expressions by the first matching pattern. Variables match any noun.
///
/// ```text
/// nock(a)             *a          // Nock of a noun is the noun itself
/// [a b c]             [a [b c]]   // Right-associative cell notation
/// ```
///
/// ### Axiomatic Functions
///
/// #### ? (wut): Cell Test
/// ```text
/// ?[a b]              0           // Returns 0 if the argument is a cell
/// ?a                  1           // Returns 1 if the argument is an atom
/// ```
///
/// #### + (lus): Increment
/// ```text
/// +[a b]              +[a b]      // Increment distributes over cells
/// +a                  1 + a       // Increment an atom
/// ```
///
/// #### = (tis): Equality Test
/// ```text
/// =[a a]              0           // Equal nouns produce 0
/// =[a b]              1           // Unequal nouns produce 1
/// ```
///
/// #### / (fas): Axis (Tree Addressing)
/// ```text
/// /[1 a]              a           // Identity
/// /[2 a b]            a           // Head of a cell
/// /[3 a b]            b           // Tail of a cell
/// /[(a + a) b]        /[2 /[a b]] // Even axes dive deeper into heads
/// /[(a + a + 1) b]    /[3 /[a b]] // Odd axes dive deeper into tails
/// /a                  /a          // Invalid address produces itself
/// ```
///
/// #### # (hax): Edit Tree
/// ```text
/// #[1 a b]            a                           // Replace entire tree
/// #[(a + a) b c]      #[a [b /[(a + a + 1) c]] c] // Edit head
/// #[(a + a + 1) b c]  #[a [/[(a + a) c] b] c]     // Edit tail
/// #a                  #a                          // Invalid edit produces itself
/// ```
///
/// ### Nock Operations
///
/// The `*` operator represents the core Nock function. It takes a subject `a` and a formula.
///
/// ```text
/// *[a [b c] d]        [*[a b c] *[a d]]   // Distribution
/// *[a 0 b]            /[b a]              // 0 is slot (axis)
/// *[a 1 b]            b                   // 1 is constant
/// *[a 2 b c]          *[*[a b] *[a c]]    // 2 is evaluate
/// *[a 3 b]            ?*[a b]             // 3 is cell test
/// *[a 4 b]            +*[a b]             // 4 is increment
/// *[a 5 b c]          =[*[a b] *[a c]]    // 5 is equality
/// ```
///
/// #### Complex Operations
///
/// ```text
/// *[a 6 b c d]        *[a *[[c d] 0 *[[2 3] 0 *[a 4 4 b]]]]   // 6 is if-then-else
/// *[a 7 b c]          *[*[a b] c]                             // 7 is compose
/// *[a 8 b c]          *[[*[a b] a] c]                         // 8 is extend
/// *[a 9 b c]          *[*[a c] 2 [0 1] 0 b]                   // 9 is invoke
/// *[a 10 [b c] d]     #[b *[a c] *[a d]]                      // 10 is hint
/// *[a 11 [b c] d]     *[[*[a c] *[a d]] 0 3]                  // 11 is hint (alternative)
/// *[a 11 b c]         *[a c]                                  // 11 is hint (simple form)
/// ```
///
/// ### Identity
///
/// ```text
/// *a                  *a          // Nock of an atom crashes
/// ```
///
/// This crate provides an implementation of the Nock 4K virtual machine, allowing you to
/// evaluate Nock expressions and build higher-level abstractions on top of this fundamental
/// combinator algebra.