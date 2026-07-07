struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }
}

fn main() {
    let rec = Rectangle {
        width: 25,
        height: 30,
    };
    println!("Area: {}", rec.area());
    println!("Perimeter: {}", rec.perimeter());
    println!("Is Square: {}", rec.is_square());

    let rec_one = Rectangle {
        width: 21,
        height: 28,
    };
    let result = rec.can_hold(&rec_one);
    println!("Can hold: {result}");
}
