extern crate fastrand;

mod person_gen;
mod names;

use person_gen::*;
use names::*;

const SEED : i64 = 0;

fn main() {
    if SEED != 0 {fastrand::seed(SEED as u64)}

    let random_person = Person::new(&choose_name(ListOption::FirstName),
                                            &choose_name(ListOption::LastName), 
                                            &choose_name(ListOption::Jobs), 
                                            fastrand::u32(18..65), 
                                            &random_gender(), 
                                            &choose_name(ListOption::Fruits));
    random_person.print_details();
}
