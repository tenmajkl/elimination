use std::{vec, collections::HashMap};

// TODO Implement swapping if there is zero in key line
fn solve(matrix: &mut Vec<Vec<i32>>) -> HashMap<usize, i32> {
    let mut current = 0;
    while current < matrix.len() {
        let mut inner = current + 1;
        while inner < matrix.len() {
            let key_mult = matrix[inner][current];
            let line_mult = -matrix[current][current];
            let mut index = current;
            while index < matrix[inner].len() {
                matrix[inner][index] = matrix[current][index] * key_mult + matrix[inner][index] * line_mult;
                index += 1;
            }
            inner += 1;
        }
        current += 1;
    }

    println!("{:?}", matrix);
    get_roots(matrix)
}

fn get_roots(matrix: &Vec<Vec<i32>>) -> HashMap<usize, i32> {
    let mut roots: HashMap<usize, i32> = HashMap::new();
    let mut line = matrix.len() - 1;
    loop {
        let mut value = matrix[line][matrix.len()];
        let mut index = matrix.len() - 1;
        while roots.get(&index).is_some() {
            value -= roots[&index] * matrix[line][index];
            index -= 1;
        }
        roots.insert(index, value / matrix[line][index]);
        if line == 0 {
            break;
        }
        line -= 1;
    }

    roots
}

fn main() {
    let mut matrix:Vec<Vec<i32>> = vec![
        vec![1, -1, -1, 5],
        vec![-1, 1, -1, 1],
        vec![-1, -1, 1, -15],
    ];
    println!("{:?}", solve(&mut matrix));
}
