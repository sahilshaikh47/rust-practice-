fn main() {
    let text = String::from("sahil shaikh 7774928747");
    let count = count_word(&text);
    println!("The word count of the strign is : {}", count )

}

fn count_word ( text: &String) -> usize {

    let  mut counter = 0;
    for ch in text.chars() {
        if ch !=  ' ' {
            counter += 1;
        }
    }
    counter
     
}