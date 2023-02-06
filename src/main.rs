enum User {
first_name(String),
last_name(String),
signed_in(bool),
}
impl User {
    
    fn display_greeting(&self) {
        if signed_in == true {
            println!("Welcome back {}", first_name);
        }
    }
}
fn main() {
    
    let first_name = User::first_name(String::from("Ivy"));
    let last_name = User::last_name(String::from("Dora"));
    let signed_in = User::signed_in(true);

    signed_in.display_greeting();
    
}
