use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn read_matrix(file_name: &str) -> Vec<Vec<f64>> {
    let mut matrix = Vec::new();
    let file = File::open(file_name).expect("Не удалось открыть файл!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Ошибка чтения строки!");
        let row: Vec<f64> = line
            .split_whitespace()
            .map(|x| x.parse::<f64>().expect("Ошибка преобразования числа!"))
            .collect();
        matrix.push(row);
    }
    matrix
}

fn write_matrix(file_name: &str, matrix: &Vec<Vec<f64>>) {
    let mut file = File::create(file_name).expect("Не удалось создать файл!");
    for row in matrix {
        let row_str: Vec<String> = row.iter().map(|x| format!("{:.6}", x)).collect();
        writeln!(file, "{}", row_str.join(" ")).expect("Ошибка записи в файл!");
    }
}
fn print_matrix(matrix: &[Vec<f64>]) {
    for row in matrix {
        for &elem in row {
            print!("{:.6} ", elem); // Выводим элемент с 6 знаками после запятой
        }
        println!(); // Переход на новую строку после каждой строки матрицы
    }
    println!("");
}
fn find_inverse(mut matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = matrix.len();
    let mut identity = vec![vec![0.0; n]; n];

    // Матрица единичка,жд
    for i in 0..n {
        identity[i][i] = 1.0;
    }

    // начало прямого хода, по лесинке
    for i in 0..n {
        let pivot = matrix[i][i];
        // println!("{}",matrix);
        print_matrix(&matrix);
        for j in 0..n {
            matrix[i][j] /= pivot;
            identity[i][j] /= pivot;
        }
        for k in (i + 1)..n {
            let factor = matrix[k][i];
            for j in 0..n {
                matrix[k][j] -= factor * matrix[i][j];
                identity[k][j] -= factor * identity[i][j];
            }
        }
    }

    // анологично но реверсе, то есть обнуляются выше диагонали 
    for i in (0..n).rev() {
        for k in 0..i {
            let factor = matrix[k][i];
            for j in 0..n {
                matrix[k][j] -= factor * matrix[i][j];
                identity[k][j] -= factor * identity[i][j];
            }
        }
    }

    identity
}

fn main() {
    let input_file = "input.txt";
    let output_file = "output.txt";

    let matrix = read_matrix(input_file);
    let inverse = find_inverse(matrix);
    write_matrix(output_file, &inverse);

    println!("Обратная матрица записана в файл {}", output_file);
}
