mod authentication;
mod text;

fn main() {
    let mut user = authentication::User::new("anna", "i4GotBADC00FFEE");
    user.set_password("4GetMe0");
    println!("Create user: {}", user.get_username());
    user.login();

    assert_eq!(text::count_letters_and_numbers("123 me"), (2, 3));
    assert_eq!(
        text::count_letters_and_numbers("221B Baker Street"),
        (12, 3)
    );
    assert_eq!(text::count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(text::count_letters_and_numbers("4 Privet Drive"), (11, 1));
}
