## 🦀 error handling in rust

<br>

* error handling in rust is different if you are coming from other languages, as it relies on `Result` and on returning instead of throwing:

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

<br>


#### ignoring the error

<br>

* use `.unwrap()`:

```rust
use std::fs;

fn main() {
  let content = fs::read_to_string("./Cargo.toml").unwrap();
  println!("{}", content)
}
```

<br>

#### terminate the program

<br>

* use `.unwrap()` or `.expect()` to trigger `panic!`:

```rust
use std::fs;

fn main() {
  let content = fs::read_to_string("./Cargo.toml").expect("Can't read Cargo.toml");
  println!("{}", content)
}
```

<br>

#### use a fallback value

<br>

* use `.unwrap()` or `.unwrap_or()`:

```rust
use std::env;

fn main() {
  let port = env::var("PORT").unwrap_or("3000".to_string());
  println!("{}", port);
}
```


<br>

#### bubble up the error

* bubble up (propagate) the error to the caller function using `?` (similar to `unwrap` but instead of panicking, it propagates the error to the calling function).

<br>

```rust
use std::collections::HashMap;

fn main() {
  match get_current_date() {
    Ok(date) => println!("We've time travelled to {}!!", date),
    Err(e) => eprintln!("Oh noes, we don't know which era we're in! :( \n  {}", e),
  }
}

fn get_current_date() -> Result<String, reqwest::Error> {
  let url = "https://postman-echo.com/time/object";
  let res = reqwest::blocking::get(url)?.json::<HashMap<String, i32>>()?;
  let date = res["years"].to_string();

  Ok(date)
}
```


<br>

#### create custom errors

<br>

* using the **[debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)** and **[display](https://doc.rust-lang.org/std/fmt/trait.Display.html)** traits:

```rust
// error.rs

// error.rs

use std::fmt;

#[derive(Debug)]
pub enum MyCustomError {
  HttpError,
  ParseError,
}

impl std::error::Error for MyCustomError {}

impl fmt::Display for MyCustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MyCustomError::HttpError => write!(f, "HTTP Error"),
      MyCustomError::ParseError => write!(f, "Parse Error"),
    }
  }
}
```