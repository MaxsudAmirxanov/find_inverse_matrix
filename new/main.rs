// fn inverse(&self) -> Option<Matrix> {
//     if !self.is_square() {
//         return None;
//     }

//     let n = self.rows;
//     let mut augmented = self.data.clone();

//     // Добавляем единичную матрицу справа от исходной
//     for i in 0..n {
//         augmented[i].extend(Matrix::identity(n).data[i].clone());
//     }

//     // Прямой ход Гаусса
//     for i in 0..n {
//         // Выбор ведущего элемента (pivoting)
//         let mut max_row = i;
//         for k in (i + 1)..n {
//             if augmented[k][i].abs() > augmented[max_row][i].abs() {
//                 max_row = k;
//             }
//         }
//         // Обмен строк, если требуется
//         if i != max_row {
//             augmented.swap(i, max_row);
//         }

//         // Проверяем, что диагональный элемент не ноль
//         if augmented[i][i].abs() < 1e-9 {
//             return None; // Матрица вырожденная
//         }

//         // Нормализуем строку
//         let diag = augmented[i][i];
//         for j in 0..(2 * n) {
//             augmented[i][j] /= diag;
//         }

//         // Обнуляем все элементы в текущем столбце, кроме диагонального
//         for k in 0..n {
//             if k != i {
//                 let factor = augmented[k][i];
//                 for j in 0..(2 * n) {
//                     augmented[k][j] -= factor * augmented[i][j];
//                 }
//             }
//         }
//     }

//     // Извлекаем обратную матрицу из правой части
//     let inverse_data: Vec<Vec<f64>> = augmented
//         .iter()
//         .map(|row| row[n..].to_vec())
//         .collect();

//     Some(Matrix::new(inverse_data))
// }



use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        // Сохдание нашей матрицы
        let rows = data.len();
        let cols = data[0].len();
        Self { data, rows, cols }
    }

    fn is_square(&self) -> bool {
        // Проверка на квадратсноть 
        self.rows == self.cols
    }

    fn identity(size: usize) -> Self {
        // Создание единичной матрицы 
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Self::new(data)
    }

    fn display(&self) {
        for row in &self.data {
            println!(
                "{:?}",
                row.iter()
                    .map(|x| format!("{:.4}", x))
                    .collect::<Vec<String>>()
            );
        }
    }

    fn inverse(&self) -> Option<Matrix> {
        if !self.is_square() {
            return None;
        }

        let n = self.rows;
        let mut augmented = self.data.clone();

        for i in 0..n {
            augmented[i].extend(Matrix::identity(n).data[i].clone());
        }

        for i in 0..n {
            let mut max_row = i;
            for k in (i + 1)..n {
                if augmented[k][i].abs() > augmented[max_row][i].abs() {
                    max_row = k;
                }
            }

            if i != max_row {
                augmented.swap(i, max_row);
            }

            if augmented[i][i].abs() < 1e-9 {
                return None;
            }

            let diag = augmented[i][i];
            for j in 0..(2 * n) {
                augmented[i][j] /= diag;
            }

            for k in 0..n {
                if k != i {
                    let factor = augmented[k][i];
                    for j in 0..(2 * n) {
                        augmented[k][j] -= factor * augmented[i][j];
                    }
                }
            }
        }

        let inverse_data: Vec<Vec<f64>> = augmented
            .iter()
            .map(|row| row[n..].to_vec())
            .collect();

        Some(Matrix::new(inverse_data))
    }
}

fn read_matrix_from_file(file_name: &str) -> io::Result<Matrix> {
    let path = Path::new(file_name);
    let file = File::open(&path)?;
    let lines = io::BufReader::new(file).lines();

    let mut data = Vec::new();
    for line in lines {
        let row: Vec<f64> = line?
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect();
        data.push(row);
    }

    Ok(Matrix::new(data))
}

fn main() -> io::Result<()> {
    let file_name = "hilbert_matrix.txt";
    let matrix = read_matrix_from_file(file_name)?;

    println!("Original Matrix:");
    matrix.display();

    match matrix.inverse() {
        Some(inverse) => {
            println!("\nInverse Matrix:");
            inverse.display();
        }
        None => println!("\nThe matrix is not invertible."),
    }

    Ok(())
}
