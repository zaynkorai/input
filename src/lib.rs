use std::io;

mod input {

    /**
    * Prompts user for a line of text from standard input and returns the
    * equivalent char; if text is not a single char, user is prompted
    * to retry. If line can't be read, returns .
    */
    pub fn get_char() -> char 
    {
        // TODO error handling
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to Input");
        buffer.trim().parse::<char>().unwrap()
    }

    /**
    * Prompts user for a line of text from standard input and returns the
    * equivalent double as precisely as possible; if text does not represent
    * a double or if value would cause underflow or overflow, user is
    * prompted to retry. If line can't be read, returns .
    */
    pub fn get_double() -> f64 {
        // TODO error handling
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to Input");
        buffer.trim().parse::<f64>().unwrap()
    }

    /**
    * Prompts user for a line of text from standard input and returns the
    * equivalent float as precisely as possible; if text does not represent
    * a float or if value would cause underflow or overflow, user is prompted
    * to retry. If line can't be read, returns .
    */
    pub fn get_float() -> f32 {
        // TODO error handling
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to Input");
        buffer.trim().parse::<f32>().unwrap()
    }

    /**
    * Prompts user for a line of text from standard input and returns the
    * equivalent int; if text does not represent an int in [-2^31, 2^31 - 1)
    * or would cause underflow or overflow, user is prompted to retry. If line
    * can't be read, returns.
    */
    pub fn get_int() -> i32 {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to Input");
        buffer.trim().parse::<i32>().unwrap()
    }

    /**
    * Prompts user for a line of text from standard input and returns the
    * equivalent long; if text does not represent a long in
    * [-2^63, 2^63 - 1) or would cause underflow or overflow, user is
    * prompted to retry. If line can't be read, returns.
    */
    pub fn get_long() -> i64 {
        // TODO error handling
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to Input");
        buffer.trim().parse::<i64>().unwrap()
    }

    /**
    * Prompts user for a line of text from standard input and returns
    * it as a string, sans trailing line ending. Supports
    * CR (\r), LF (\n), and CRLF (\r\n) as line endings. If user
    * inputs only a line ending, returns "", not NULL. Returns NULL
    * upon error or no input whatsoever (i.e., just EOF). Stores string
    * on heap.
    */
    pub fn get_string()-> String {
        // TODO error handling
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to Input");
        buffer
    }

    /**
    * Prompts user for a line of text from standard input with predefine text and returns
    * it as a string, sans trailing line ending. Supports
    * CR (\r), LF (\n), and CRLF (\r\n) as line endings. If user
    * inputs only a line ending, returns "", not NULL. Returns NULL
    * upon error or no input whatsoever (i.e., just EOF). Stores string
    * on heap.
    */

    pub fn input(message:String) -> String {
        // TODO error handling
        print!("{} ", message);
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to Input");
        buffer
    }

}