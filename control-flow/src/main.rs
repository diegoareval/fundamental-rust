fn main() {
    conditioning();
    looping();
    returning_values_loop();
    do_while_loop();
    looping_through_collection();
}

fn conditioning(){
    let number = 6; 
    if number % 4 == 0 {
        println!("number is divisible by 4");
    }else if number % 3 == 0{
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("nothing")
    }

    let condition = true;
    let value = if condition {5} else {6};
    println!("value found is: {}", value);
}

fn looping(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop { 
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn returning_values_loop(){
  let mut counter = 0;
  let result =  loop  {
      counter +=1;
      if counter == 10{
          // break the loop returning values
          break counter * 2;
      }
  };

  println!("The result is {}", result);
}

fn do_while_loop(){
    let mut number = 3;

    while number!= 0 {
        println!("{}", number);
        number -=1;
    }

    println!("done!!!")
}

fn looping_through_collection(){
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
        let mut index = 0;

            while index < months.len() - 1 {
                println!("month is: {}", months[index]);
                index += 1;
            }
        println!("finished");
        // looping with for
        for element in months {
            println!("the value is: {}", element);
        }

        for value in (1..10).rev() {
            println!("{}!", value);
        }


}
