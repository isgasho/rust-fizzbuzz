pub struct FizzBuzz {

}

impl FizzBuzz {
    pub fn say_it(input: u32) -> String {
        if input % 3 == 0 {
            return String::from("Fizz");
        }
        return input.to_string();
    }
}


pub fn say_it_external(input: u32) -> String {
    return input.to_string();
}