fn main() {
    let  s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("word: {}", word);

    {
        let  s = String::from("hello world");
        let count = second_word(&s);
        println!("count: {}", count);
    }
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();
  for(i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
        return &s[0..i];
      }
  }
  &s[..]
}

fn second_word(s: &String) -> usize {


    // acceding from position 0 to position 5
    let hello = &s[0..5];
    println!("hello: {}", hello);
    // acceding from position 6 to position 11
    let world = &s[6..11];
    println!("world: {}", world);

    // acceding from position 0 to position 2
    let slice = &s[0..2];
    println!("slice 1: {}", slice);

    // acceding from beggining to position 2
    let slice = &s[..2];
    println!("slice 2: {}", slice);

    // length of the string
    let len = s.len();

    // acceding from position 3 to the end of the string
    let slice = &s[3..len];
    println!("slice 3: {}", slice);

    // acceding from the position 3 to the end
    let slice = &s[3..];
    println!("slice 4: {}", slice);
    slice.len()


}