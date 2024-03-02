// fn main() {
//     println!("Hello, world!");//the invocation with the bang indicates that we are calling a MACRO and not a FUNCTION
// }

enum Transport {
    Water,
    Road,
    Rail
}

struct City {
    sample_name: String,
    description: String,
    is_coastal: bool,
    population: i32,
    main_transport: Transport
}

fn new_city(population: i32, is_coastal: bool) -> City {
    if is_coastal {
        City {
            sample_name: format!("Lagos"),
            main_transport: if is_coastal { Transport::Water } else { Transport::Road },
            description: format!("The City of lagos is a densely populated coastal city in south-western Nigeria, with {} residents as at the last census.", population),
            population,
            is_coastal
        }
    } else {
        City {
            sample_name: format!("Enugu"),
            main_transport: if is_coastal { Transport::Water } else { Transport::Road },
            description: format!("The City of Enugu is located in Enugu State, south-eastern Nigeria. It has a population of {}.", population),
            population,
            is_coastal,
        }
    }
}

fn main() {
    let city_with_beach = new_city(12332499, true);
    println!("{}", city_with_beach.description);

    let landlocked_city = new_city(5200000, false);
    println!("{}", landlocked_city.description);
}