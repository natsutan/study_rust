#[derive(Default)]
struct PizzaConfig {
    wants_cheese: bool,
    number_of_olives: i32,
    special_message: String,
    crust_type: CrustType
}

enum CrustType {
    Thin,
    Thick
}

impl Default for CrustType {
    fn default() -> CrustType {
        CrustType::Thin
    }
}

fn main() {

    let foo: i32 = Default::default();
    println!("foo = {}", foo);

    let pizza: PizzaConfig = Default::default();
    println!("want cheese {}", pizza.wants_cheese);
    println!("olive {}", pizza.number_of_olives);
    println!("messege {}", pizza.special_message);

    let crust_type =
        match pizza.crust_type {
            CrustType::Thin => "Nice and Thin",
            CrustType::Thick => "Extra Thick"
        };

    println!("crust {}", crust_type);
}