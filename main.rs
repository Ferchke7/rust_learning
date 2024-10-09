
enum Phones {
    Number(u64),
    Text(String)
}
fn main() {
    let v: Vec<i32> = Vec::new();
    let c = vec![1,2,3];
    let j: Vec<i32> = vec![];
    //default fill the values
    let vs = vec![5;3];
    let mut users = Vec::new();
    users.push("Tom");
    users.push("Sam");
    users.push("Bob");
    users[0] = "FER";

    let secondindex = users.get(1);
    match secondindex {
        Some(user) => println!("is user {}", user),
        None => println!("User not found")
    }   
    for user in &users{
        println!("{}", user);
    }
    let phones = vec![
        Phones::Number(123131213),
        Phones::Text(String::from("soetete"))
    ];

    for phone in &phones {
        match phone {
            Phones::Number(n) => println!("{n}"),
            Phones::Text(x) => println!("{x}")
        }
    }
}