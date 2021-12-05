struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) ->u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {


   let width1 = 30;
   let height1 = 50;

   println!(
       "The area of the rectangle is {} square pixels.",
       area(width1, height1)
   );

   let rect1 = (30, 50);
   println!(
    "The area01 of the rectangle is {} square pixels.",
    area01(rect1)
);

let rect1 = Rectangle {
    width: 30,
    height: 50,
};

println!(
    "The area of the rectangle with impl is {} square pixels.",
    rect1.area()
);


println!(
    "The area of the rectangle is {} square pixels.",
    area03(&rect1)
);

methods();

}


// fn build_user(email: String, username: String) -> User {
//     User {
//         username,
//         email: email,
//         active: true,
//         sign_in_account: 1
//     }
// }

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area01(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area03(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn methods(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

