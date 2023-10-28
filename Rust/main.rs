use std::io::{self, Write};

struct Point {
    x: f64,
    y: f64,
    z: f64,
}

const PROMPT_CHOICE: &str = "Enter your choice (1 for Euclidean, 2 for Manhattan): ";

fn main() {
    let point1 = get_coordinates("Enter the first point (x1 y1 z1): ")
        .expect("Failed to get coordinates for point 1");
    let point2 = get_coordinates("Enter the second point (x2 y2 z2): ")
        .expect("Failed to get coordinates for point 2");

    match get_distance_calculation_choice() {
        Ok(1) => {
            let euclidean_distance = euclidean_distance(&point1, &point2);
            println!("Euclidean Distance: {:.2}", euclidean_distance);
        }
        Ok(2) => {
            let manhattan_distance = manhattan_distance(&point1, &point2);
            println!("Manhattan Distance: {:.2}", manhattan_distance);
        }
        _ => println!("Invalid choice. Please select 1 or 2."),
    }
}

fn get_coordinates(prompt: &str) -> io::Result<Point> {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let coordinates: Result<Vec<f64>, _> =
            input.split_whitespace().map(|s| s.trim().parse()).collect();

        match coordinates {
            Ok(coords) if coords.len() == 3 => {
                return Ok(Point {
                    x: coords[0],
                    y: coords[1],
                    z: coords[2],
                });
            }
            _ => println!("Invalid input. Please enter three numbers separated by spaces."),
        }
    }
}

fn get_distance_calculation_choice() -> io::Result<u32> {
    loop {
        print!("{}", PROMPT_CHOICE);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse() {
            Ok(1) | Ok(2) => return Ok(input.trim().parse().unwrap()),
            _ => {
                println!("Invalid choice. Please select 1 or 2.");
            }
        }
    }
}

fn calculate_differences(point1: &Point, point2: &Point) -> Point {
    Point {
        x: point2.x - point1.x,
        y: point2.y - point1.y,
        z: point2.z - point1.z,
    }
}

fn euclidean_distance(point1: &Point, point2: &Point) -> f64 {
    let diff = calculate_differences(point1, point2);
    (diff.x.powi(2) + diff.y.powi(2) + diff.z.powi(2)).sqrt()
}

fn manhattan_distance(point1: &Point, point2: &Point) -> f64 {
    let diff = calculate_differences(point1, point2);
    diff.x.abs() + diff.y.abs() + diff.z.abs()
}
