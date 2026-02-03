# rust-practice-  1


Rust Character Counter
Overview

Counts characters excluding spaces in a string.

Code Purpose

Input:

"sahil shaikh 7774928747"


Output:

The word count of the string is : 22

Methods Used

Method                            / Concept	Use
String::from()	                   Creates an owned, heap-allocated string
&text	                           orrows string without transferring ownership
Function count_word()	           Separates counting logic
println!()	                       Prints formatted output
&String parameter	               Accepts reference to avoid copying
usize	                           Stores count values
mut counter	                       Allows variable to change
chars()	                           Converts the string into an iterator of characters, because a String  cannot be looped over directly; Rust strings are UTF-8 bytes, so chars () ensures proper character iteration
for loop	                       Traverses characters
if ch != ' '	                   Skips spaces
Return value	                   Last expression returns count