/* Write a function called checkTriangleType that takes three parameters representing the
lengths of the sides of a triangle. The function should return a string indicating the type of triangle: "equilateral", "isosceles", or "scalene". */



fn main() {
    println!("{}",check_triangle_type(3.0, 3.0, 3.0));
    println!("{}",check_triangle_type(3.1, 3.1, 5.6));
    println!("{}",check_triangle_type(1.2, 3.4, 7.8)); 
}

fn check_triangle_type(a: f32, b: f32, c: f32) -> String {
    if a + b > c && a + c > b && b + c > a {
        if a==b && b==c {
            String::from("equilateral")
        } else if a==b || b==c || a==c {
            String::from("isosceles")
        }else {
            String::from("scalene")
        }
    } else {
        String::from("not a triangle")
    }
}
