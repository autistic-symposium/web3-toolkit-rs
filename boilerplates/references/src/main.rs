fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {

    // simple reference
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("references are {} {}", res1,res2);

    // mutable reference
    let mut res = 4.0;
    modifies(&mut res);
    println!("references are {}", res);
}