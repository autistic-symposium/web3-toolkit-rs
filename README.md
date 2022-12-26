## 🥷🏻🎸⛓️ playing pvp in the metaweb: rust edition

<br>

### tl; dr 

##### why rust

* ahead-of-time compiled language (compiling and running are separate steps)
* easy control of low-level assets (such as memory usage)
* better visibility for concurrency bugs.

##### features

* a crate is a compilation unit in Rust (i.e. file.rs is a crate file).
* paths in `use` statements are relative to the current module, unless they begin with the name of a crate.
* `println!` is a macro, not a function (strings are checked at compile-time)
* `::` syntax indicates that the right side is an associated function of the left side (static method)
* you can always run `cargo check` to validate your code, folder structure, etc. 
* you can alwas run `rustc --explain <error>` to understand errors.


<br>

---

### in this repo

* [📚 web3-toolkit](web3toolkit)
    - an ongoing development of a library and set of rust scripts with my fav on-chain ops.
<br>


----

### resources

##### basics

* [install](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch01-01-installation.html)
* [cargo](https://doc.rust-lang.org/cargo/): dep manager and build tool
* [rustfmt](https://github.com/rust-lang/rustfmt): code style

<br>

##### learning


* [rust docs](https://doc.rust-lang.org/stable/book/)
* [mit rust docs](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/index.html)
* [gentle intro to rust](https://stevedonovan.github.io/rust-gentle-intro/readme.html)
* [ethereum foundation on rust](https://ethereum.org/en/developers/docs/programming-languages/rust/)


<br>

##### libraries

* [web3 rust library](https://github.com/tomusdrw/rust-web3)
* [tokio async library](https://tokio.rs/tokio/tutorial/async)
