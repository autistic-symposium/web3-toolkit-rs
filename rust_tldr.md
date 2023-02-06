## 🤯 rust tl; dr

<br>

### why rust

* ahead-of-time compiled language (compiling and running are separate steps).
* easy control of low-level assets (such as memory usage).
* better visibility for concurrency bugs.

<br>

---
### features


* a `crate` is a compilation unit in Rust (i.e. `file.rs` is a crate file). it can be lib or a bin.
* paths in `use` statements are relative to the current module, unless they begin with the name of a crate.
* `println!` is a macro, not a function (strings are checked at compile-time)
* `::` syntax indicates that the right side is an associated function of the left side (static method)
* `cargo new` creates a binary; `cargo new --lib`, a library.
* you can always run `cargo check` to validate your code, folder structure, etc. 
* you can alwas run `rustc --explain <error>` to understand errors.




<br>

---
### data structures

* an array is a collection of objects of the same type T, stored in contiguous memory, represented by `[]`.
* custom types are `struct` and `enum`
* `From` and `Into` traits are inherently linked, providing a simple mechanism for converting between several types.
* the type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a hashing function.s

#### variable bindings

* safety via static typing (variable bindings can be type annotated when declared - and the compiler will silence warnings when vars are prefixed by `_`).
* variable bindings are immutable but can be mutable with `mut` modifier.

#### smart pointers

* smart pointers: while references only borrow data, in many cases, smart pointers own the data they point to. 
* examples: `String` (stores its capacity as metadata and has the extra ability to ensure its data will always be valid UTF-8) and `Vec<T>`. 
* usually implemented using structs, but unlike an ordinary struct, smart pointers implement the `Deref` and `Drop` traits.
* `Deref` allows an instance of the smart pointer struct to behave like a reference so we can write code to work with either references or smart pointers.
* `Drop` allows customizing the code that's run when an instance of the smart pointer goes out of scope.


<br>

---

### flow of control

#### if/else

```
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
```

#### loops

```
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
```

#### while

```    
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

```

#### for

```
    for n in 1..101 { // or  for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
```

#### match

* same pattern as C's `switch`:

```
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("no matchl"),
    }

```

* a `match` guard can be added to filter the arm.
* a `match` block can [destructure items in a variety of ways](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html).

#### expressions

* blocks are expressions: they can be used as values in assignments. 
* the last expression in the block will be assigned to the place expression, such as a local variable if no `;`, otherwise the return value will be `()`.

<br>

---

### functions


#### associated functions

* methods are associated functions that are called on a particular instance of a type.
* associated functions are functions that are defined on a type generally (like constructors):

```
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}
```

#### [closure](https://doc.rust-lang.org/rust-by-example/fn/closures/anonymity.html)

* calling a closure is exactly like calling a function. however, both input and return types can be inferred and input variable names must be specified.
* `||` instead of `()` around input variables.
* closures are anonymous.

```
    let closure_annotated = |i: i32| -> i32 { i + 1 };
```

* closures can capture variables:
    - by reference: `&T`
    - by mutable reference: `&mut T`
    - by value: `T`

* when taking closure as input parameters, the closure's complete type must be annotated:
    - `Fn`: the closure uses the captured value by reference (`&T`).
    - `FnMut`: the closure uses the captured value by mutable reference (`&mut T`).
    - `FnOnce`: the closure uses the captured value by value (`T`).

#### diverging functions

* diverging functions never return. 
* they are marked using `!`, which is an empty type:

```
fn foo() -> ! {
    panic!("This call never returns.");
}
```

<br>

---
### modules

* a module is a collection of items: functions, structs, traits, `impl` blocks, and other modules.
* only public items of a module can be accessed from outside the module scope.
* `use` declaration can be used to bind a full path to a new name, for easer access.
* the `super` and `self` keywords can be used in the path to remove ambiguity when accessing items.
* [(OMG file hierarchy in rust is poetry 🖤)](https://doc.rust-lang.org/rust-by-example/mod/split.html).

<br>

---
### generics

* a type parameter is specified as generic by the use of angle brackets and upper camel cases (`<Aaa>`).
* traits are similar to a feature often called interfaces in other languages, although with some differences.
* every reference in rust has a lifetime, which is the scope for which the reference is valid.

<br>

---

### memory

* memory in rust is managed through a system of ownership with a set of rules that the compiler checks.
* in rust, a value is either on the stack (LIFO, faster) on the heap (linked list, slower). 
* The `String` type is stored on the heap.
* No garbage collector, the memory is automatically returned once the variable that owns it goes out of scope. 

<br>


---
### projects

* a crate is the smallest amount of code that the compiler considers at time.
* a package can contain multiple binary crates and optionally one library crate. each binary must have a `main` function. librady crates don't have a `main` function. 
* most of the times when we say "crate" we mean library crate.
* a package is a bundle of one or more crates that provide a set of functionality, and contains a `Cargo.toml` describing how to build the crates.
* when compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
* in the crate root file, if you declare a "garden" with `mod garden`, the compiler looks for:
    - inline, within curly brackets that replace the semicolon following `mod garden`
    - in the file `src/garden.rs`
    - in the file `src/garden/mod.rs`

<br>

---

### concurrency


* concurrency: different parts of a program execute independently. parallel programming: different parts of a program execute at the same time.
* ownership and type system are a set of tools to help manage memory safety and concurrency problems/bugs.
* to accomplish message-sending concurrency, Rust's standard library provides `channels`. a channel is a concept by which data is sent from one thread to another.
