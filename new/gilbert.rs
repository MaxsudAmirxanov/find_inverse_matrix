use std::fs::File;
use std::io::{self, Write};

/// Функция для создания матрицы Гильберта
fn create_hilbert_matrix(size: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; size]; size];
    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = 1.0 / ((i + j + 1) as f64); // H(i, j) = 1 / (i + j - 1)
        }
    }
    matrix
}

/// Функция для записи матрицы в файл
fn write_matrix_to_file(matrix: &[Vec<f64>], file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    for row in matrix {
        let row_str: String = row.iter()
            .map(|x| format!("{:.6}", x)) // Округляем до 6 знаков
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(file, "{}", row_str)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // Спрашиваем размер матрицы у пользователя
    println!("Введите размер матрицы Гильберта:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let size: usize = input.trim().parse().expect("Введите положительное целое число!");

    // Генерируем матрицу Гильберта
    let hilbert_matrix = create_hilbert_matrix(size);

    // Вывод матрицы на экран
    println!("Сгенерированная матрица Гильберта:");
    for row in &hilbert_matrix {
        println!("{:?}", row.iter().map(|x| format!("{:.6}", x)).collect::<Vec<String>>());
    }

    // Записываем матрицу в файл
    let file_name = "hilbert_matrix.txt";
    write_matrix_to_file(&hilbert_matrix, file_name)?;
    println!("\nМатрица успешно записана в файл: {}", file_name);

    Ok(())
}
