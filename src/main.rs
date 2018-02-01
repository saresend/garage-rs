
extern crate clap;

use clap::{App, Arg};



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







}
