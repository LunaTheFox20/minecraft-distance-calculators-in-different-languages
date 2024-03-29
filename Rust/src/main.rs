use std::io::{self, BufRead, Write};

const PROMPT_CHOICE: &str = "Enter your choice (1 for Euclidean, 2 for Manhattan): ";

#[repr(C)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    let point1 = get_point("Enter the first point (x1 y1 z1): ", &mut stdin_lock)?;
    let point2 = get_point("Enter the second point (x2 y2 z2): ", &mut stdin_lock)?;

    match get_distance_calculation_choice(&mut stdin_lock)? {
        1 => {
            let euclidean_distance = euclidean_distance(point1, point2);
            println!("Euclidean Distance: {:.2}", euclidean_distance);
        }
        2 => {
            let manhattan_distance = manhattan_distance(point1, point2);
            println!("Manhattan Distance: {:.2}", manhattan_distance);
        }
        _ => println!("Invalid choice. Please select 1 or 2."),
    }

    Ok(())
}

fn get_user_input(prompt: &str, stdin: &mut impl BufRead) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    Ok(input)
}

fn get_point(prompt: &str, stdin: &mut impl BufRead) -> io::Result<Point> {
    loop {
        let input = get_user_input(prompt, stdin)?;
        let coordinates: Vec<_> = input.split_whitespace().map(|s| s.parse()).collect();

        match coordinates.as_slice() {
            [Ok(x), Ok(y), Ok(z)] => {
                return Ok(Point {
                    x: *x,
                    y: *y,
                    z: *z,
                })
            }
            _ => println!("Invalid input. Please enter three numbers separated by spaces."),
        }
    }
}

fn get_distance_calculation_choice(stdin: &mut impl BufRead) -> io::Result<u32> {
    loop {
        let input = get_user_input(PROMPT_CHOICE, stdin)?;
        match input.trim().parse() {
            Ok(1) | Ok(2) => return Ok(input.trim().parse().unwrap()),
            _ => println!("Invalid choice. Please select 1 or 2."),
        }
    }
}

fn euclidean_distance(point1: Point, point2: Point) -> f64 {
    let x_diff = point2.x - point1.x;
    let y_diff = point2.y - point1.y;
    let z_diff = point2.z - point1.z;
    (x_diff.powi(2) + y_diff.powi(2) + z_diff.powi(2)).sqrt()
}

fn manhattan_distance(point1: Point, point2: Point) -> f64 {
    let x_diff = point2.x - point1.x;
    let y_diff = point2.y - point1.y;
    let z_diff = point2.z - point1.z;
    x_diff.abs() + y_diff.abs() + z_diff.abs()
}
