pub struct Person {
    first_name: String,
    last_name: Option<String>,
    occupation: String,
    age: u32,
    gender: String,
    favourite_fruit: Option<String>,
}

impl Person {
    pub fn new(first_name: &str, last_name: &str, occupation: &str, age: u32, gender: &str, favourite_fruit: &str) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: Some(last_name.to_string()),
            occupation: occupation.to_string(),
            age,
            gender: gender.to_string(),
            favourite_fruit: Some(favourite_fruit.to_string()),
        }
    }

    pub fn print_details(&self) {
        let full_name: String = self.concat_name();

        println!("Name: {}", full_name);
        println!("Occupation: {}", self.occupation);
        println!("Age: {}", self.age);

        let fruit = if (&self).favourite_fruit.is_none() {
            "None"
        }
        else {
            &self.favourite_fruit.as_ref().unwrap()
        };

        println!("Gender: {}", self.gender);
        println!("Favourite Fruit: {}\n", fruit)
    }

    fn concat_name(&self) -> String {
        if (&self).last_name.is_none() {
            String::from(&(&self).first_name)
        } 
        else {
            String::from(&self.first_name) + " " + (&self).last_name.as_ref().unwrap()
        }
    }
}