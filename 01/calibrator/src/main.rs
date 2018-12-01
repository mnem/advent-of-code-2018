use std::fs;

fn main() {
    let input_filename = String::from("input.txt");
    let input = fs::read_to_string(input_filename)
        .expect("Failed to read file");

    let mut total_calibration = 0;
    for line in input.lines() {
        let calibration: i32 = line.parse().unwrap();
        total_calibration += calibration
    }

    println!("Final calibration: {}\n", total_calibration);
}
