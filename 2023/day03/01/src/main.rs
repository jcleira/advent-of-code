use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = match File::open("input.txt") {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    let mut matrix = Vec::new();
    let mut matrix_tracker = Vec::new();

    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();

        let mut row = Vec::new();
        let mut row_tracker = Vec::new();
        for c in line.chars() {
            row.push(c);
            row_tracker.push(false);
        }
        matrix.push(row);
        matrix_tracker.push(row_tracker);
    }

    let result = explore_matrix(&matrix, &mut matrix_tracker);

    println!("result: {}", result);
}

fn explore_matrix(matrix: &Vec<Vec<char>>, matrix_tracker: &mut Vec<Vec<bool>>) -> i32 {
    let mut sum = 0;
    for (x, row) in matrix.iter().enumerate() {
        for (y, item) in row.iter().enumerate() {
            if is_symbol(*item) {
                println!("");
                println!("found symbol at {} {} {}", x, y, item);
                sum += sum_adjacent_numers(&matrix, matrix_tracker, x, y);
                println!("sum {}", sum);
            }
        }
    }

    sum
}

fn sum_adjacent_numers(
    matrix: &Vec<Vec<char>>,
    matrix_tracker: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
) -> i32 {
    let mut x_min = x;
    if x_min > 0 {
        x_min -= 1;
    }
    let mut x_max = x;
    if x_max < matrix[x].len() - 1 {
        x_max += 1;
    }

    let mut y_min = y;
    if y_min > 0 {
        y_min -= 1;
    }

    let mut y_max = y;
    if y_max < matrix.len() {
        y_max += 1;
    }

    println!("indexes {} {} {} {}", x_min, x_max, y_min, y_max);

    let mut sum = 0;
    for i in x_min..x_max + 1 {
        for j in y_min..y_max + 1 {
            if matrix[i][j].is_numeric() {
                sum += build_number(&matrix, matrix_tracker, i, j);
                println!("sum {}", sum);
            }
        }
    }

    sum
}

fn build_number(
    matrix: &Vec<Vec<char>>,
    matrix_tracker: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
) -> i32 {
    let mut i = y;
    let mut j = y;

    loop {
        if i == 0 {
            break;
        }

        if matrix[x][i - 1].is_numeric() {
            i -= 1;
        } else {
            break;
        }
    }

    loop {
        if j == matrix[x].len() {
            break;
        }

        if matrix[x][j].is_numeric() {
            j += 1;
        } else {
            break;
        }
    }

    println!("number indexes {} {}", i, j);

    for i in i..j {
        if matrix_tracker[x][i] {
            println!("number already built");
            return 0;
        }
    }

    let mut number_string = String::new();
    for index in i..j {
        println!("index {}", index);
        number_string.push(matrix[x][index]);
        matrix_tracker[x][index] = true;
    }

    if number_string.len() == 0 {
        return 0;
    }

    number_string.parse::<i32>().unwrap()
}

fn is_symbol(c: char) -> bool {
    !c.is_alphabetic() && !c.is_numeric() && c != '.'
}
