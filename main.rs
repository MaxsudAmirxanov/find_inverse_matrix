use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

/// Чтение матрицы из файла
fn read_matrix_from_file(filename: &str) -> Result<Vec<Vec<f64>>, String> {
    let path = Path::new(filename);
    let file = File::open(&path).map_err(|_| format!("Не удалось открыть файл: {}", filename))?;
    let reader = io::BufReader::new(file);

    let mut matrix = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|_| "Ошибка чтения строки из файла.".to_string())?;
        let row: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse::<f64>().map_err(|_| "Ошибка преобразования в число.".to_string()))
            .collect::<Result<_, _>>()?;
        matrix.push(row);
    }

    Ok(matrix)
}

/// Запись матрицы в файл
fn write_matrix_to_file(filename: &str, matrix: &[Vec<f64>]) -> Result<(), String> {
    let path = Path::new(filename);
    let mut file = File::create(&path).map_err(|_| format!("Не удалось создать файл: {}", filename))?;
    for row in matrix {
        let row_str: Vec<String> = row.iter().map(|v| format!("{:.6}", v)).collect();
        writeln!(file, "{}", row_str.join(" ")).map_err(|_| "Ошибка записи в файл.".to_string())?;
    }
    Ok(())
}

/// Метод Гаусса для вычисления обратной матрицы
fn gaussian_elimination(matrix: &mut Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, String> {
    let n = matrix.len();
    let mut extended_matrix = vec![vec![0.0; 2 * n]; n];

    // Создаем расширенную матрицу с единичной матрицей
    for i in 0..n {
        for j in 0..n {
            extended_matrix[i][j] = matrix[i][j];
        }
        extended_matrix[i][n + i] = 1.0;
    }

    // Прямой ход
    for i in 0..n {
        // Поиск максимального элемента в текущем столбце
        let mut max_row = i;
        for k in i + 1..n {
            if extended_matrix[k][i].abs() > extended_matrix[max_row][i].abs() {
                max_row = k;
            }
        }

        // Проверка на вырожденность
        if extended_matrix[max_row][i] == 0.0 {
            return Err("Матрица вырожденная, обратная не существует.".to_string());
        }

        // Перестановка строк
        extended_matrix.swap(i, max_row);

        // Нормализация строки
        let pivot = extended_matrix[i][i];
        for j in 0..2 * n {
            extended_matrix[i][j] /= pivot;
        }

        // Обнуление всех остальных строк в текущем столбце
        for k in 0..n {
            if k != i {
                let multiplier = extended_matrix[k][i];
                for j in 0..2 * n {
                    extended_matrix[k][j] -= multiplier * extended_matrix[i][j];
                }
            }
        }
    }

    // Извлечение обратной матрицы (правая часть расширенной матрицы)
    let inverse_matrix = extended_matrix
        .iter()
        .map(|row| row[n..2 * n].to_vec())
        .collect();

    Ok(inverse_matrix)
}

fn main() {
    let input_file = "input.txt";
    let output_file = "output.txt";

    match read_matrix_from_file(input_file) {
        Ok(mut matrix) => {
            println!("Чтение матрицы из файла: {} завершено.", input_file);

            match gaussian_elimination(&mut matrix) {
                Ok(inverse_matrix) => {
                    if let Err(e) = write_matrix_to_file(output_file, &inverse_matrix) {
                        eprintln!("Ошибка записи в файл: {}", e);
                    } else {
                        println!("Обратная матрица успешно записана в файл: {}", output_file);
                    }
                }
                Err(e) => eprintln!("Ошибка вычисления обратной матрицы: {}", e),
            }
        }
        Err(e) => eprintln!("Ошибка чтения матрицы: {}", e),
    }
}
