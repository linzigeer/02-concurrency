fn main() {
    let mut a1 = 10u32;
    let mut b = &mut a1;
    let mut c = &mut b;
    let d = &mut c;
    ***d = 30;
    println!("{d}");

    // let mut e = 10;
    // e = 20;
    // let f = &e;
    // println!("{}", f);
}
