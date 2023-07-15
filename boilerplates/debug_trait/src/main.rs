use std::fmt;

struct Person {
    first_name: String,
    last_name: String,
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}


fn main() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Snow".to_string(),
    };
    println!("{:?}", p);
}
