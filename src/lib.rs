mod tests {

    pub fn hello() -> String {
        ("Hello, world!").to_string()
    }

    pub fn get_char(){
    
    }
    pub fn get_double(){
    
    }
    pub fn get_float(){
    
    }
    pub fn get_int(){
    
    }
    pub fn get_long(){
    
    }
    pub fn get_string(){
    
}

}
#[cfg(test)]
mod tests {

    use super::input::hello;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}