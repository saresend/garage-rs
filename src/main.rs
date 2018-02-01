
extern crate clap;
#[macro_use]
extern crate diesel;


use clap::{App, Arg};
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;

fn main() {

    let matches = App::new("Garage - quick CLI datastore")
        .version("0.1")
        .author("Samuel Resendez <saresend@usc.edu>")
        .about("Easy way to store string values for later use!")
        .arg(
            Arg::with_name("CMD")
                .help("Accepts either s for set variable, or g for get variable")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("KEYNAME")
                .help("The Key Value to Get or Set")
                .required(true)
                .index(2),
        )
        .get_matches();


    let command = matches.value_of("CMD").unwrap();
    let key_name = matches.value_of("KEYNAME").unwrap();

    println!("{} {}", command, key_name);

    match command {

        "s" => handle_set(key_name),
        "g" => handle_get(key_name),
        _ => println!("Invalid CMD, please enter s for Set, or g for GET"),

    }

}

fn establish_connection() -> SqliteConnection {

    let database_url = String::from("data.sqlite");
    SqliteConnection::establish(&database_url).unwrap()
}


fn handle_set(key_name: &str) {
    let connection = establish_connection();


}

fn handle_get(key_name: &str) {}
