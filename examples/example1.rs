fn main() {
    let mut s = "hello，世界".to_string();

    let x = s.as_bytes();
    println!("{:?}", x);

    println!("{:?}", "世".as_bytes());
    println!("{:?}", "界".as_bytes());
    let char = s.pop().unwrap();
    println!("popped char:{}", char);
    println!("as bytes:{:?}", char.to_string().as_bytes());
    let byte_escape = "I'm saying hello.\0";
    println!("{}", byte_escape);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);
}
