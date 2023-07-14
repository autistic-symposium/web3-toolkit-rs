## command line arguments

<br>


### `std::env::args` 

<br>

* `std::env::args` is how you access command-line arguments. it returns an iterator over the arguments as strings:

```rust
fn main() {
    for arg in std::env::args() {
        println!("'{}'", arg);
    }
}
```

* you can also use `Vec` and `collect` to make that a vector, and using the iterator `skip` method to move:

```rust
 let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 { 
        ...
    }
```

* or using a single argument together with parsing an integer value:

```rust
use std::env;

fn main() {
    let first = env::args().nth(1).expect("please supply an argument");
    let n: i32 = first.parse().expect("not an integer!");
    // do stuff
}
```

<br>

### `clap`

* the [clap](https://docs.rs/clap/latest/clap/) crate makes thing easier.
