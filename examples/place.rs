extern crate egg_mode;

mod common;

use egg_mode::place::PlaceType;

fn main() {
    let config = common::Config::load();

    let result = egg_mode::place::search_query("columbia")
                                 .granularity(PlaceType::Admin)
                                 .max_results(10)
                                 .call(&config.con_token, &config.access_token).unwrap();

    println!("{} results for \"columbia\", administrative regions or larger:", result.response.results.len());

    for place in &result.response.results {
        println!("{}", place.full_name);
    }
    println!("");

    let result = egg_mode::place::reverse_geocode(51.507222, -0.1275)
                                 .granularity(PlaceType::City)
                                 .call(&config.con_token, &config.access_token).unwrap();

    println!("{} results for reverse-geocoding {}, {}:", result.response.results.len(),
                                                         51.507222, -0.1275);

    for place in &result.response.results {
        println!("{}", place.full_name);
    }
}
