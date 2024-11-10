#include <iostream>
#include <vector>
#include <cmath>
#include <iomanip>

using namespace std;
using Matrix = vector<vector<double>>;

// Функция для получения обратной матрицы методом Гаусса
bool invertMatrix(const Matrix &A, Matrix &inverse) {
    int n = A.size();
    inverse.assign(n, vector<double>(n, 0.0));
    
    // Создаем расширенную матрицу [A|I]
    Matrix augmented(n, vector<double>(2 * n, 0.0));
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            augmented[i][j] = A[i][j];
        }
        augmented[i][i + n] = 1.0;
    }

    // Прямой ход
    for (int i = 0; i < n; i++) {
        // Поиск максимального элемента для численной устойчивости
        double maxElem = fabs(augmented[i][i]);
        int maxRow = i;
        for (int k = i + 1; k < n; k++) {
            if (fabs(augmented[k][i]) > maxElem) {
                maxElem = fabs(augmented[k][i]);
                maxRow = k;
            }
        }

        // Обмен строками для избежания деления на малые числа
        if (i != maxRow) {
            swap(augmented[i], augmented[maxRow]);
        }

        // Приведение диагонального элемента к 1
        double pivot = augmented[i][i];
        if (fabs(pivot) < 1e-10) {
            cerr << "Матрица необратима." << endl;
            return false;
        }
        for (int j = 0; j < 2 * n; j++) {
            augmented[i][j] /= pivot;
        }

        // Обнуление элементов столбца
        for (int k = 0; k < n; k++) {
            if (k != i) {
                double factor = augmented[k][i];
                for (int j = 0; j < 2 * n; j++) {
                    augmented[k][j] -= factor * augmented[i][j];
                }
            }
        }
    }

    // Извлечение обратной матрицы из расширенной матрицы
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            inverse[i][j] = augmented[i][j + n];
        }
    }

    return true;
}

// Функция для печати матрицы
void printMatrix(const Matrix &matrix) {
    for (const auto &row : matrix) {
        for (double value : row) {
            cout << setw(10) << value << " ";
        }
        cout << endl;
    }
}

int main() {
    int n;
    cout << "Vedite razmer matrici: ";
    cin >> n;

    Matrix A(n, vector<double>(n)), inverse;
    cout << "Vedite elementi matrici:" << endl;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            cin >> A[i][j];
        }
    }

    if (invertMatrix(A, inverse)) {
        cout << "Obratnaya matrica:" << endl;
        printMatrix(inverse);
    } else {
        cout << "Matrica neobrotima." << endl;
    }

    return 0;
}
