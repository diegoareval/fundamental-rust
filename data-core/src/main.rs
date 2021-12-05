use std::io;

fn main() {
    let a: u8 = 123; // u= unasigned; 8 bits
    println!("a: {}", a); // inmutable

    let mut b: i8 = 0; //signed: -128 - 127
    println!("b: {} before", b); // mutable
    b = 18;
    println!("b: {} after", b);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // declare constants
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    // we can redefine same variable inside the scope
    {
        let a: u8 = 12;
        println!("a inside: {}", a); 
    }

    let spaces = "     ";
    println!("spaces len: {}", spaces.len());

    // basic operations
    {
        let sum = 5 + 10;
        println!("sum: {}", sum);

        let mult = 5*5;
        println!("mult: {}", mult);

        let div = 10/2;
        println!("div: {}", div);

        let rest = 10 - 5;
        println!("rest = {}", rest)
    }

    // tup
    {
        let _tup: (i32, f64, u8) = (500, 6.4, 1);
        let _tup = (500, 6.4, 1);

    let (x, y, z) = _tup;

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);
    }

    // arrays
    {
        let a = [1, 2, 3, 4, 5];

        let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
              println!("val a = {}",  a[0]);

              println!("month number 12 is = {}",  months[months.len() - 1]);
       
    }

    // get the number
    {
        let a = [1, 2, 3, 4, 5];
        // label to type in a data
        println!("Please enter an array index.");
       
        // receive a new data and put in a variable
        let mut index = String::new();
        
        // read the line with the index
        io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

         // check if the index is valid
        let index: usize = index.trim().parse().expect("this index is invalid");
       
        // acceding some elements based on a index
        let element = a[index];

        // print the element
         println!("the value of element at index {} is {}", index, element);
    }
}
