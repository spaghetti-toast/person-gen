extern crate lazy_static;
extern crate fastrand;

use lazy_static::lazy_static;
use std::{
    path::Path,
    fs::File,
    io::{self, BufRead},
};

pub enum ListOption {
    FirstName,
    LastName,
    Jobs,
    Fruits,
}

// PATHS
const FIRST_NAME_LIST_PATH: &str = "lib/names.txt";
const LAST_NAME_LIST_PATH: &str = "lib/last-names.txt";
const JOBS_LIST_PATH: &str = "lib/jobs.txt";
const FRUITS_LIST_PATH: &str = "lib/fruits.txt";


lazy_static! {
    static ref FIRST_NAME_LIST: Vec<String> = read_lines_to_vec(&FIRST_NAME_LIST_PATH);
    static ref LAST_NAME_LIST: Vec<String> = read_lines_to_vec(&LAST_NAME_LIST_PATH);
    static ref JOBS_LIST: Vec<String> = read_lines_to_vec(&JOBS_LIST_PATH);
    static ref FRUITS_LIST: Vec<String> = read_lines_to_vec(&FRUITS_LIST_PATH);
}


/// Function to choose a random String from a specified list
/// 
/// # Arguments
/// 
/// * `list` - Takes an enum value of ListOption
pub fn choose_name(list: ListOption) -> String {
    match list {
        ListOption::FirstName => title_case(choose_random_string(&FIRST_NAME_LIST)),
        ListOption::LastName => title_case(choose_random_string(&LAST_NAME_LIST)),
        ListOption::Jobs => choose_random_string(&JOBS_LIST).trim().to_string(),
        ListOption::Fruits => choose_random_string(&FRUITS_LIST),
    }
}

/// Function to return a random String from a vector of Strings using `fastrand`.
/// 
/// # Arguments
///
/// * `input_vector` - A vector slice that holds a list of strings
/// 
/// https://stackoverflow.com/questions/24102615/passing-a-vec-into-a-function-by-reference
pub fn choose_random_string(input_vector : &Vec<String>) -> String {
    input_vector[fastrand::usize(..input_vector.len())].to_string()
}

/// Function to return gender string
/// 
/// # Returns
/// 
/// * `"Male"`
/// * `"Female"`
/// * `"Other"`
pub fn random_gender() -> String {
    match fastrand::u8(0..=2) {
        0 => "Male".to_string(),
        1 => "Female".to_string(),
        2 => "Other".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Reads lines from a file using BufReader and returns a vector with each name
/// as a seperate item in the list. 
///
/// # Arguments
/// 
/// * `filepath` - A path within the user filesystem, can be entered as a constant or by polling user
///                input
fn read_lines_to_vec(filepath : &dyn AsRef<Path>) -> Vec<String>{
    let mut vector: Vec<String> = vec![];

    let file_result = File::open(filepath);
    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let lines = io::BufReader::new(file).lines();

    for line in lines {
        vector.push(line.expect("Problem adding line to vector"));
    }

    vector
}

/// Function to return a formatted string with an uppercase first letter, with the rest of
/// the word lowercase. 
/// 
/// # Arguments
/// 
/// * `input_string` - A string consisting of a single word
fn title_case(input_string: String) -> String {
    let first_letter = &input_string[0..1];
    let first_letter = &first_letter.to_uppercase();
    
    let rest_of_the_word = &input_string[1..];
    let rest_of_the_word = &rest_of_the_word.to_lowercase();

    first_letter.to_string() + &rest_of_the_word
}