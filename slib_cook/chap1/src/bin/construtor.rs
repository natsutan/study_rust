struct NameLength {
    name: String,
    length: usize,
}

impl NameLength {
    fn new(name: &str) -> Self {
        NameLength {
            length: name.len(),
            name: name.to_string(),
        }
    }

    fn print(&self) {
        println!("name {}, len = {}", self.name, self.length);

    }

}

fn main()
{
    let name_len = NameLength::new("Natu");
    name_len.print();
}