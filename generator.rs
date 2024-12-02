use std::fs::File;
use std::io::{self, Write};
use rand::Rng;

/// Генерация случайной матрицы и запись её в файл
fn generate_matrix_to_file(filename: &str, size: usize, range: (i32, i32)) -> io::Result<()> {
    let mut file = File::create(filename)?;

    let mut rng = rand::thread_rng();

    for _ in 0..size {
        let row: Vec<String> = (0..size)
            .map(|_| rng.gen_range(range.0..=range.1).to_string())
            .collect();
        writeln!(file, "{}", row.join(" "))?;
    }

    Ok(())
}

fn main() {
    let filename = "input.txt"; // Имя выходного файла
    let size = 1000;           // Размер матрицы (например, 1000x1000)
    let range = (-100, 100);   // Диапазон случайных чисел

    match generate_matrix_to_file(filename, size, range) {
        Ok(_) => println!("Случайная матрица размером {}x{} записана в файл '{}'.", size, size, filename),
        Err(e) => eprintln!("Ошибка при генерации матрицы: {}", e),
    }
}
