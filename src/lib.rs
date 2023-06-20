#![allow(unused, dead_code)]

//Welcome to the library crate root! You found it!

//The front of house is all parts of a restuarant that directly concern the customer
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
                                            
        pub fn seat_at_table() {}
    }
        
    pub mod serving {
        use crate::back_of_house;
        use back_of_house::Meal;

        pub fn take_order() {
           back_of_house::add_order_to_waitlist();
        }

        pub fn serve_order(order: Meal) {
            println!("Here's your meal, dear customer:");

            match order {
                Meal::Breakfast(b) => {
                    println!("Some {} toast with seasonal fruits", b.toast);
                    //Doesn't work! seasonal_fruit is a private field
                    // println!("The seasonal fruits are {}", b.seasonal_fruit);
                }
                Meal::Soda { brand, oz } => println!("An ice cold, {oz} ounce of {brand} soda"),
            }
        }
            
        pub fn take_payment() {
            println!("Thank you for your service!");
        }
    }
}

//Organization.
//It's called 'crate root' because the contents of it form a module named crate at the root of the mod structure

//Alright, now let's say we want to call a function in this module. How?

pub fn ask_for_table() {
    //We could use the absolute path:
    crate::front_of_house::hosting::add_to_waitlist();

    //or we could use the relative path:
    //This is okay because front_of_house is defined in the same module as front_of_house
    front_of_house::hosting::seat_at_table();

    //They'll both throw an error unless the stuff that's called is defined as public (Rust has privacy by default)
    //Children can hide stuff from parents, but not the other way around
    //Everything (methods, constants, mods, structs, enums) has to be declared public to be used from outside

    //The public API is how users interact with the code in a crate
    //There are many decisions to be made to make your code easier to use: check out Rust API Guidelines to see
}


//Best practices for packages with a binary crate (src\main.rs) and a library (src\lib.rs):
/* 
    - If a package has both types of crate root, 
        - usually the binary crate has just enough code to start an executable that calls code with the lib crate

    - Module tree defined in lib crate, then any public items can be accessed through the binary crate
        - That way, the binary crate is like an external crate: it can only use the public API
*/


//Using super to start relative paths

//Super starts a path at the parent module (like starting a path with ..)
//Used if:
    //we know an item is in the parent module and could be moved in module system, 
    //and the child module is closely related and therefore would be moved with it

fn deliver_order() {}


mod back_of_house {
    //Imagine a restaurant where the fruit served with your breakfast is based on the season

    //In a pub enum, the fields are either all public or all private
    enum Season { Winter, Spring, Summer, Autumn, }  

    //But in a pub struct, the fields are private by default
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    //If a struct has private fields, you must provide a constructor to create an instance of that struct
    impl Breakfast {
        fn cook(toast: &str, season: Season) -> Breakfast {
            let seasonal_fruit: String = match season {
                Season::Winter => String::from("mandarins"),
                Season::Spring => String::from("strawberries"),
                Season::Summer => String::from("cantaloupe"),
                Season::Autumn => String::from("apples"),
            };
    
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit,
            }
        }
    }

    pub enum Meal {
        Breakfast(Breakfast),
        Soda { brand: String, oz: u8},
    }

    pub fn add_order_to_waitlist() {
        super::front_of_house::serving::serve_order(cook_order());
    }

    fn cook_order() -> Meal {
        let breakfast: Breakfast = Breakfast::cook("whole wheat", Season::Autumn);
        Meal::Breakfast(breakfast)
    }

    fn fix_incorrect_order() {
        //cook_order is a sibling of fix_incorrect_order, so it can be called straight with a relative path
        cook_order();
        //deliver_order is a sibling of back_of_house, the parent of fix_incorrect_order, so it can be called thru super
        super::deliver_order();
    }
}

pub fn eat_at_restaurant() {
    ask_for_table();
    front_of_house::serving::take_order();
}