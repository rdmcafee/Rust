fn main() {
    
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate(){
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }
    let mut s = String::from("Happiness is foreign");
    let new_word = first_word(&s);
    s.clear();
    println!("first word of phrase has: {} characters", new_word);
    
}
