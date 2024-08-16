mod matrix;

fn main() {
    let mut a = "hello world".to_string();
    a.push_str(" Oscar");
    println!("{}", a);

    let mut vec = Vec::with_capacity(4);
    vec.push(Option::Some("a"));
    vec.push(Option::Some("b"));
    vec.push(Option::Some("c"));

    let b = vec[1].take();
    println!("{b:?}");

    let mut user = User {
        name: "Oscar".into(),
        age: 18,
        sign_count: 20,
        address: Some("中国四川成都".into()),
    };

    let address = user.address.take().unwrap();
    // 不能直接使用user.name然后还想使用user，因为user的name为String类型，没有实现Copy，这里会被move，导致user partially moved
    // 注意，你可以使用user.name来给其他变量赋值，但是这里会导致user partially moved，user不完整，无法再使用，
    // 如果你后面不再使用user，或者给user.name重新赋值，那么其实还是可以的
    // let name = user.name;
    let name1 = user.name.clone();
    let name2 = user.name;
    user.name = "Hello".into();
    println!("{address:?}");
    println!("{name1:?}");
    println!("{name2:?}");
    println!("{user:?}");
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub sign_count: u16,
    pub address: Option<String>,
}
