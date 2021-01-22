pub mod utils;
use std::cmp::max;
fn main() {
    let vars = utils::list_env::get_all_environment_variables();
    println!("");
    if vars.is_some() {
        let unwrapped = vars.unwrap();
        if unwrapped.len() == 0 {
            return;
        }

        let mut name_length = 0;
        let mut value_length = 0;
        let mut declared_length = 0;
        for var in &unwrapped {
            name_length = max(name_length, var.get_name().len());
            value_length = max(value_length, var.get_value().len());
            declared_length = max(declared_length, var.get_declared_in().len());
        }

        // todo optional column names

        for mut var in unwrapped {
            var.balance_lengths(name_length, value_length, declared_length);
            println!("{}", var);
        }
    }
}
