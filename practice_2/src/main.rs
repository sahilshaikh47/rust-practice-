fn main ( ) {
    let string_to_be_reversed = String::from("sahil shaikh");
    let reveresed_string = reverse_string(&string_to_be_reversed);
    println!("reversed string is :> {}",reveresed_string)
    
}

fn reverse_string ( text: &String)  -> String {

    let chars: Vec<char> = text.chars().collect();
    let mut reversed = String::new();

    let mut i = chars.len();
    
    while i > 0 {
        i-= 1;
        reversed.push(chars[i])
    }
    reversed

   
 
}





// Overview

// Converts a string into a vector of characters by consuming an iterator.

// Code Purpose

// Input

// text = "sahil shaikh"


// Operation explanation 

// let chars: Vec<char> = text.chars().collect();


// Output

// ['s','a','h','i','l',' ','s','h','a','i','k','h']  



// Methods /                      Concepts Used
// Keyword /                             Concept	Use
// fn main()	                         Program entry point
// String::from()	                     Creates owned heap string
// &String	                             Borrows string reference
// Function reverse_string()            	Encapsulates reversal logic
// -> String	                          Function returns owned string
// Vec<char>	                          Stores characters for indexing
// chars()	                              Converts string into character iterator
// collect()	                          Gathers iterator into vector
// String::new()                          Creates empty result string
// mut	                                  Allows variable mutation
// len()                                  Gets number of characters
// usize	                              Type used for indexing
// while loop	                          Manual reverse traversal
// i -= 1	                              decrement index
// push()	                              Appends character to string
// Indexing [i]	                          Access vector element
// Return value                        	 Last expression returns result




