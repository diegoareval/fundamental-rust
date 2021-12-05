fn main() {

    let s1 = String::from("hello");

let (value, length) = calculate_len(s1);
println!("value is: {} with length: {}", value, length);

let mut s = String::from("hello");
change(&mut s);

let ownership = gives_ownership();
println!("ownership is: {}", ownership);

}


fn calculate_len(value: String) -> (String, usize) {
    let length = value.len();
    (value, length)
}

fn gives_ownership() -> String {  
// assign to new variable and returns ir
let some_string = String::from("yours"); 
   some_string 
}


fn change(some_string: &mut String){
    some_string.push_str("mundo!!");
    println!("value = {}", some_string);
}