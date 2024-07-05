use std::env;
use grep_lib;

fn main() {
    // Collects user's inputs.
    let user_input : Vec<String> = env::args().collect();

    // Creates instance of Grep object to call the function.
    let run_grep = grep_lib::Grep::set_details(&user_input);

    // Runs the grep function from 'lib.rs'.
    run_grep.grep();
}
