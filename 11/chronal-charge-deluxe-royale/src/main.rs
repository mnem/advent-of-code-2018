// Use: https://en.wikipedia.org/wiki/Summed-area_table

struct

fn main() {
    let result = process(2568);
    println!("Result: ({},{}) side: {}, power: {}\n", result.0, result.1, result.3, result.2);
}

fn process(serial_number: i64) -> (i64, i64, i64, i64) {
    let grid = charge_grid(serial_number);
    max_power(&grid)
}

fn charge_grid(serial_number: i64) -> Vec<i64> {
    let side = 300usize;
    let mut grid = vec![0i64; side * side];
    for y in 0..side {
        for x in 0..side {
            grid[y * side + x] = power_level(x as i64, y as i64, serial_number);
        }
    }

    return grid;
}

fn sample_grid(grid: &Vec<i64>, start_x: i64, start_y: i64, side: i64) -> i64 {
    let mut total = 0i64;
    let start_x= start_x as usize;
    let start_y= start_y as usize;
    for y in start_y..(start_y + side as usize) {
        for x in start_x..(start_x + side as usize) {
            total += grid[y * 300 + x];
        }
    }
    return total;
}

fn power_level(x: i64, y: i64, serial: i64) -> i64 {
    let rack_id = x + 10;
    let intermediate_power = (rack_id * y + serial) * rack_id;
    let intermediate_power_str = intermediate_power.to_string();
    let base_power: i64 = match intermediate_power_str.chars().nth(intermediate_power_str.len() - 3) {
        Some(c) => c.to_string().parse().expect("Expected a digit"),
        None => 0,
    };
    return base_power - 5;
}

fn max_power_for_size(grid: &Vec<i64>, side: i64) -> (i64, i64, i64) {
    let mut power = i64::min_value();
    let mut power_x = 0i64;
    let mut power_y = 0i64;
    for y in 0..(300 - side) {
        for x in 0..(300 - side) {
            let sample_power = sample_grid(grid, x, y, side);
            if sample_power > power {
                power = sample_power;
                power_x = x;
                power_y = y;
            }
        }
    }

    (power_x, power_y, power)
}

fn max_power(grid: &Vec<i64>) -> (i64, i64, i64, i64) {
    let mut max_power = 0i64;
    let mut max_side = 0i64;
    let mut max_x = 0i64;
    let mut max_y = 0i64;

    for side in 1..=300 {
        let (x, y, power) = max_power_for_size(&grid, side);
        if power > max_power {
            max_power = power;
            max_side = side;
            max_x = x;
            max_y = y;
        }
    }

    return (max_x, max_y, max_power, max_side );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        let result = power_level(3, 5, 8);
        assert_eq!(4, result);
    }

    #[test]
    fn test_example_exact_1() {
        let grid = charge_grid(18);
        let result = sample_grid(&grid, 33, 45, 3);
        assert_eq!(29, result);
    }

    #[test]
    fn test_example_exact_2() {
        let grid = charge_grid(42);
        let result = sample_grid(&grid, 21, 61, 3);
        assert_eq!(30, result);
    }

    #[test]
    fn test_example_1() {
        let grid = charge_grid(18);
        let (x, y, power, side) = max_power(&grid);
        assert_eq!(90, x);
        assert_eq!(269, y);
        assert_eq!(113, power);
        assert_eq!(16, side);
    }

    #[test]
    fn test_example_2() {
        let grid = charge_grid(42);
        let (x, y, power, side) = max_power(&grid);
        assert_eq!(232, x);
        assert_eq!(251, y);
        assert_eq!(119, power);
        assert_eq!(12, side);
    }
}
