fn main() {
    sum(12,12);
    res(12,12);
    mult(12,12);
    div(12,12);
    data_with_label(12, "my label");
    hooding();

    let value = returning_values();
    println!("value: {}", value);
}

fn sum(a: i32, b: i32){
  println!("sum is: {}", a + b);
}

fn res(a: i32, b: i32){
    println!("resta is: {}", a - b);
  }

  fn mult(a: i32, b: i32){
    println!("mult is: {}", a * b);
  }

  fn div(a: i32, b: i32){
    println!("div is: {}", a / b);
  }

  fn data_with_label(value: i32, label: &str){
      println!("value: {} with the label: {}", value, label);
  }

  fn hooding(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

  }

  fn returning_values() -> i32 {
      5
  }


