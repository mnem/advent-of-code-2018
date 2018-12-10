use std::fs;

fn main() {
    let input = read_input();
    let result = process(&input);
    println!("Result: {}\n", result);
}

fn read_input() -> String {
    let input_filename = String::from("input.txt");
    fs::read_to_string(input_filename)
        .expect("Failed to read file")
}

fn process(input: &str) -> u32 {
    let mut data = input_to_vec(&input);
    let node =  make_node(&mut data);

    return node.value();
}

fn input_to_vec(input: &str) -> Vec<u32> {
    let mut vec: Vec<u32> = input.trim().split(" ").map(|s| { s.parse().unwrap() }).collect();
    vec.reverse();
    vec
}

fn make_node(input: &mut Vec<u32>) -> Node {
    let num_children = input.pop().unwrap();
    let num_metadatum = input.pop().unwrap();

    let mut children = Vec::new();
    for _ in 0..num_children {
        children.push(Box::new(make_node(input)));
    }

    let mut metadata = Vec::new();
    for _ in 0..num_metadatum {
        metadata.push(input.pop().unwrap());
    }

    Node {children, metadata}
}

struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<u32>,
}

impl Node {
    #[cfg(test)]
    fn num_all_children(&self) -> usize {
        let mut num_all_children: usize = 0;
        for child in &self.children {
            num_all_children += child.num_all_children();
        }
        return num_all_children + self.children.len();
    }

    #[cfg(test)]
    fn sum_metadata(&self) -> u32 {
        let mut sum: u32 = self.metadata.iter().sum();

        for child in &self.children {
            sum += child.sum_metadata();
        }

        return sum;
    }

    fn value(&self) -> u32 {
        if self.children.len() == 0 {
            return self.metadata.iter().sum();
        }

        let mut value = 0u32;
        for data in &self.metadata {
            if *data == 0 {
                continue;
            }

            let index = (*data - 1) as usize;
            if index >= self.children.len() {
                continue;
            }

            value += self.children[index].value();
        }

        return value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_to_vec() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let result = input_to_vec(input);

        let mut expected = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2,];
        expected.reverse();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_vec_to_node() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let mut vec = input_to_vec(input);
        let result = make_node(&mut vec);
        assert_eq!(3, result.num_all_children());
        assert_eq!(138, result.sum_metadata());
        assert_eq!(66, result.value());
    }
}
