
struct Burger {
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool
}

impl Burger {
    fn print(&self) {
        let pretty_patties =
            match self.patty_count {
                1 => "patty",
                _ => "patties"
            };
        let pretty_bool = |val|  if val { "" } else { "no "};
        let pretty_vegetarian = if self.vegetarian {"vegetarian " } else {""} ;

        println!("this is a {} burger with {} {}, {} cheese, {}bacon and {} salad.",
            pretty_vegetarian,
                 self.patty_count,
            pretty_patties,
            pretty_bool(self.cheese),
            pretty_bool(self.bacon),
            pretty_bool(self.salad)
        )
    }
}

struct BurgerBuilder {
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool
}

impl BurgerBuilder {
    fn new() -> Self {
        BurgerBuilder {
            patty_count: 1,
            vegetarian: false,
            cheese: false,
            bacon: false,
            salad: true
        }
    }
    fn patty_count(mut self, val: i32) -> Self {
        self.patty_count = val;
        self
    }

    fn vegetarian(mut self, val: bool) -> Self {
        self.vegetarian = val;
        self
    }

    fn cheese(mut self, val: bool) -> Self {
        self.cheese = val;
        self
    }

    fn bacon(mut self, val: bool) -> Self {
        self.bacon = val;
        self
    }

    fn salad(mut self, val: bool) -> Self {
        self.salad = val;
        self
    }

    fn build(&self) -> Result<Burger, String> {
        let burger = Burger {
            patty_count: self.patty_count,
            vegetarian: self.vegetarian,
            cheese: self.cheese,
            bacon: self.bacon,
            salad: self.salad
        };

        if burger.vegetarian && burger.bacon {
            Err("Soory,".to_string())
        } else {
            Ok(burger)
        }
    }
}

fn main()
{
    let normal_burger = BurgerBuilder::new().build();
    let cheese_burger = BurgerBuilder::new().cheese(true).salad(false).build();
    let veggie_burger = BurgerBuilder::new().vegetarian(true).patty_count(2).build();

    if let Ok(normal_burger) = normal_burger {
        normal_burger.print();
    }
    if let Ok(cheese_burger) = cheese_burger {
        cheese_burger.print();
    }
    if let Ok(veggie_burger) = veggie_burger {
        veggie_burger.print();
    }

    let invalid_burger = BurgerBuilder::new().vegetarian(true).bacon(true).build();
    if let Err(error) = invalid_burger {
        println!("Fail to print. {}", error);
    }

    let cheese_burger_builder = BurgerBuilder::new().cheese(true);
    for i in 1..10 {
        let cheese_burger = cheese_burger_builder.build();
        if let Ok(_cheese_burger) = cheese_burger {
            println!("cheese burger number {} is ready.", i);
        }
    }


}

