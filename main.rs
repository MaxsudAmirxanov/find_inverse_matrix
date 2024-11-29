use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Data {
    file_name: String,
}

impl Data {
    fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
        }
    }

    fn get_data(&self) -> io::Result<Vec<String>> {
        let path = Path::new(&self.file_name);
        let file = File::open(&path)?;
        let lines = io::BufReader::new(file).lines();
        lines.collect()
    }

    fn return_matrix(&self) -> io::Result<Vec<Vec<i32>>> {
        let data = self.get_data()?;
        let mut matrix = Vec::new();

        for line in data {
            let mut row = Vec::new();
            for ch in line.chars() {
                if let Ok(num) = ch.to_string().parse::<i32>() {
                    row.push(num);
                }
            }
            if !row.is_empty() {
                matrix.push(row);
            }
        }

        Ok(matrix)
    }

    fn draw_matrix(matrix: &[Vec<i32>]) {
        for row in matrix {
            println!("{:?}", row);
        }
    }
}

struct Matrix {
    matrix: Vec<Vec<i32>>,
}

impl Matrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self { matrix }
    }

    fn create_single_matrix(&self) -> Vec<Vec<i32>> {
        let rows = self.matrix.len();
        let cols = self.matrix[0].len();
        let mut augmented_matrix = self.matrix.clone();

        for i in 0..rows {
            let mut identity_row = vec![0; cols];
            identity_row[i] = 1;
            augmented_matrix[i].extend(identity_row);
        }

        augmented_matrix
    }
}

fn main() -> io::Result<()> {
    let data = Data::new("input.txt");

    let original_matrix = data.return_matrix()?;
    println!("Original Matrix:");
    Data::draw_matrix(&original_matrix);

    let matrix = Matrix::new(original_matrix);
    let augmented_matrix = matrix.create_single_matrix();

    println!("\nAugmented Matrix:");
    Data::draw_matrix(&augmented_matrix);

    Ok(())
}
