use lazy_static::lazy_static;
use std::collections::HashMap;


lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("Lilith", vec!["user", "admin"]);
        map.insert("Osiris", vec!["user"]);
        map
    };
}

fn show_access(name: &str) {
    let access = PRIVILEGES.get(name);
    println!("{}: {:?}", name, access);
}


fn main() {
    let access = PRIVILEGES.get("Lilith");
    println!("Lilith: {:?}", access);

    show_access("Adam");
}
