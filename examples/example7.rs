#[warn(unused_assignments)]
fn main() {
    let a = String::from("a");
    // let mut b = String::from("b");

    let c = &a;
    // #[warn(unused_assignments)]
    // let mut d = &a;
    // d = &mut b;

    println!("{}", c);
    // println!("{}", d);
}
