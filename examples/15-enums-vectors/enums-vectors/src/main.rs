enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64),
}

fn find_largest_shape(shapes: &[Shape]) -> Option<&Shape> {
    shapes.iter().max_by(|a, b| {
        let area_a = match a {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base) => 0.5 * base * base,
        };
        let area_b = match b {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base) => 0.5 * base * base,
        };
        area_a.partial_cmp(&area_b).unwrap()
    })
}

fn main() {
    let shapes = vec![Shape::Circle(3.0), Shape::Square(7.0), Shape::Triangle(3.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base) => 0.5 * base * base,
        })
        .sum();

    println!("Total area: {} sq. units", total_area);

    if let Some(largest_shape) = find_largest_shape(&shapes) {
        match largest_shape {
            Shape::Circle(radius) => println!("Largest shape: Circle with radius {}", radius),
            Shape::Square(length) => println!("Largest shape: Square with length {}", length),
            Shape::Triangle(base) => println!("Largest shape: Triangle with base {}", base),
        }
    } else {
        println!("No shapes found");
    }
}

// Modify the code to add a new variant to the `Shape` enum, representing a Triangle shape. Update the `main` function to include a triangle shape in the `shapes` vector and calculate the total area considering the triangle's dimensions.
// Extend the program by implementing a function that takes a vector of shapes as input and returns the largest shape based on its area. Invoke this function with the `shapes` vector in the `main` function and print the details of the largest shape.
