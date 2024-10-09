// &i32        // обычная ссылка
// &'a i32     // ссылка с указанным временем жизни
// &'a mut i32 // изменяемая ссылка с указанным временем жизни

fn main() {
    let message = get_message();
    println!("message: {}", message);
}

fn get_message<'a>() -> &'a str {
    "hello"
}