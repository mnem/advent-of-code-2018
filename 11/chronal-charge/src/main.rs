fn main() {
    let result = process(2568);
    println!("Result: {},{} (power: {})\n", result.0, result.1, result.2);
}

fn process(serial_number: i64) -> (i64, i64, i64) {
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

fn sample_grid(grid: &Vec<i64>, x: i64, y: i64) -> i64 {
    let y1 = (((y + 0) * 300) + x) as usize;
    let y2 = (((y + 1) * 300) + x) as usize;
    let y3 = (((y + 2) * 300) + x) as usize;

    grid[y1 + 0] + grid[y1 + 1] + grid[y1 + 2] +
    grid[y2 + 0] + grid[y2 + 1] + grid[y2 + 2] +
    grid[y3 + 0] + grid[y3 + 1] + grid[y3 + 2]
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

fn max_power(grid: &Vec<i64>) -> (i64, i64, i64) {
    let mut power = i64::min_value();
    let mut power_x = 0i64;
    let mut power_y = 0i64;
    for y in 0..(300 - 3 as i64) {
        for x in 0..(300 - 3 as i64) {
            let sample_power = sample_grid(grid, x, y);
            if sample_power > power {
                power = sample_power;
                power_x = x;
                power_y = y;
            }
        }
    }

    (power_x, power_y, power)
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
        let result = sample_grid(&grid, 33, 45);
        assert_eq!(29, result);
    }

    #[test]
    fn test_example_exact_2() {
        let grid = charge_grid(42);
        let result = sample_grid(&grid, 21, 61);
        assert_eq!(30, result);
    }

    #[test]
    fn test_example_1() {
        let grid = charge_grid(18);
        let (x, y, power) = max_power(&grid);
        assert_eq!(33, x);
        assert_eq!(45, y);
        assert_eq!(29, power);
    }

    #[test]
    fn test_example_2() {
        let grid = charge_grid(42);
        let (x, y, power) = max_power(&grid);
        assert_eq!(21, x);
        assert_eq!(61, y);
        assert_eq!(30, power);
    }
}
