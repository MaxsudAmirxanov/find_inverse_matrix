use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

/// Чтение строки матрицы из файла
fn read_row(file_path: &str, row_index: usize) -> io::Result<Vec<f64>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if index == row_index {
            let row = line?;
            return Ok(row
                .split_whitespace()
                .filter_map(|num| num.parse::<f64>().ok())
                .collect());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Row index out of bounds",
    ))
}

/// Запись строки в файл (обновление строки)
/// Запись строки в файл (обновление строки)
fn write_row(file_path: &str, row_index: usize, new_row: &[f64]) -> io::Result<()> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect();

    let formatted_row = new_row
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    if row_index < lines.len() {
        lines[row_index] = formatted_row;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Row index out of bounds",
        ));
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

/// Прямой ход метода Гаусса-Жордана
fn gauss_jordan_elimination(file_path: &str) -> io::Result<()> {
    let (n, m) = get_matrix_size(file_path)?;
    if m != 2 * n {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Matrix is not properly augmented for Gauss-Jordan elimination",
        ));
    }

    for i in 0..n {
        // 1. Нормализуем текущую строку
        let mut row = read_row(file_path, i)?;
        let diag = row[i];
        for j in 0..m {
            row[j] /= diag;
        }
        write_row(file_path, i, &row)?;

        // 2. Обнуляем остальные строки в текущем столбце
        for k in 0..n {
            if k != i {
                let mut other_row = read_row(file_path, k)?;
                let factor = other_row[i];
                for j in 0..m {
                    other_row[j] -= factor * row[j];
                }
                write_row(file_path, k, &other_row)?;
            }
        }
    }

    Ok(())
}


/// Возвращает количество строк и столбцов матрицы
fn get_matrix_size(file_path: &str) -> io::Result<(usize, usize)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let rows = reader.lines().count();

    let file = File::open(file_path)?;
    let first_line = BufReader::new(file).lines().next();
    let cols = if let Some(Ok(line)) = first_line {
        line.split_whitespace().count()
    } else {
        0
    };

    Ok((rows, cols))
}

/// Создаёт единичную матрицу размером `n x n` и возвращает её как список строк
fn create_identity_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut identity = vec![vec![0.0; n]; n];
    for i in 0..n {
        identity[i][i] = 1.0;
    }
    identity
}

/// Добавляет единичную матрицу к основной (расширяет матрицу)
fn augment_matrix(file_path: &str, output_path: &str) -> io::Result<()> {
    let (rows, cols) = get_matrix_size(file_path)?;
    if rows != cols {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Matrix must be square to augment with identity matrix",
        ));
    }

    let identity = create_identity_matrix(rows);

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    for (index, line) in reader.lines().enumerate() {
        let mut row: Vec<f64> = line?
            .split_whitespace()
            .filter_map(|num| num.parse::<f64>().ok())
            .collect();

        row.extend(&identity[index]);
        writeln!(
            writer,
            "{}",
            row.iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )?;
    }

    Ok(())
}

/// Прямой ход метода Гаусса-Жордана
fn gauss_jordan_elimination(file_path: &str) -> io::Result<()> {
    let (n, m) = get_matrix_size(file_path)?;
    if m != 2 * n {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Matrix is not properly augmented for Gauss-Jordan elimination",
        ));
    }

    let mut file = File::open(file_path)?;
    for i in 0..n {
        // 1. Нормализуем текущую строку
        let mut row = read_row(file_path, i)?;
        let diag = row[i];
        for j in 0..m {
            row[j] /= diag;
        }
        write_row(file_path, i, row)?;

        // 2. Обнуляем остальные строки в текущем столбце
        for k in 0..n {
            if k != i {
                let mut other_row = read_row(file_path, k)?;
                let factor = other_row[i];
                for j in 0..m {
                    other_row[j] -= factor * row[j];
                }
                write_row(file_path, k, other_row)?;
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let input_file = "matrix.txt";
    let augmented_file = "augmented_matrix.txt";

    // Создаём расширенную матрицу
    augment_matrix(input_file, augmented_file)?;
    println!("Augmented matrix written to {}", augmented_file);

    // Выполняем метод Гаусса-Жордана
    gauss_jordan_elimination(augmented_file)?;
    println!("Gauss-Jordan elimination completed. Result written to {}", augmented_file);

    Ok(())
}
