#[derive(Debug)]
struct Point<T, U> {    // Can have multiple generic types.
    x : T,
    y : U,
}


#[derive(Debug)]
enum _MyOption<T> {        // `Option` is the best example.
    Some(T),
    None,
}


fn main() {
    
    // Generic functions.
    let int_vec = vec![0, 3, 5, 2, 9];
    println!("{}", largest(&int_vec));

    // Generic structs.
    let int_point = Point { x : 3, y : 4 };
    let float_point = Point { x : 7.98, y : 2.17 };
    let int_and_float_point = Point { x : 3, y : 2.17 };
    println!("{:?} {:?} {:?}", int_point, float_point, int_and_float_point);
    println!("{:?} {:?}", int_point.x_as_ref(), float_point.mix_up(&int_and_float_point));
    println!("{}", float_point.distance_from_origin());

    // Generic enums.
    let res1 = Some(3);
    let res2 = Some(Point { x : 3, y : 4 });
    println!("{:?} {:?}", res1, res2);
}


// Define generic functions.
fn largest<T : PartialOrd + Copy>(list : &[T]) -> T {    // Specify trait bounds.
    let mut largest = list[0];  // Move happens here. So `T` needs to be copyable.
    for &item in list {
        if item > largest {     // Partial compare happens here. So `T` needs to be orderable.
            largest = item;
        }
    }
    largest
}


// Implement generic methods for generic structs.
impl<T : Copy, U : Copy> Point<T, U> {

    fn x_as_ref(&self) -> &T {
        &self.x
    }

    fn mix_up<V : Copy, W : Copy>(&self, other : &Point<V, W>) -> Point<T, W> {    // Takes other generic parameters.
        Point { x : self.x, y : other.y }
    }
}


impl Point<f32, f32> {  // Only implement for type `Point<f32, f32>`.

    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
