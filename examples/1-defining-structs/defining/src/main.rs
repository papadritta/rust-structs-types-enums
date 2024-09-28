#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: String,
}
/*
impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
*/
fn main() {
    let alfredo = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: "mega-rust@gmail.com".to_string(),
        phone_number: "+1-123455-66-77".to_string(),
    };

    println!("{:?}", alfredo);
    // println!("Full Name: {}", person.full_name());
}
