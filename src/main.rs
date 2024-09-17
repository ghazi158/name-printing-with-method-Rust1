#[derive(Debug)]
struct FullName {
    first: String,
    last: String,
}

impl FullName {
    fn printname(&self) {
        println!(
            "My first name is: {} and my last name is: {}",
            self.first, self.last
        );
    }
}
fn main() {
    let username = FullName {
        first: String::from("Ghazi"),
        last: String::from("foo"),
    };

    username.printname();
}
