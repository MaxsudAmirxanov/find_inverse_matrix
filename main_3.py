class Data:
    def __init__(self, file_name) -> None:
        self.file_name = file_name

    def get_data(self):
        """Чтение данных из файла."""
        try:
            with open(f"{self.file_name}.txt", "r") as f:
                lines = f.readlines()
        except FileNotFoundError:
            print(f"Файл {self.file_name}.txt не найден.")
            lines = []
        return lines

    def return_matrix(self):
        """Преобразование данных из файла в числовую матрицу."""
        data = self.get_data()
        matrix = []
        for line in data:
            try:
                row = [int(elem) for elem in line.strip().split()]
                matrix.append(row)
            except ValueError:
                print(f"Ошибка преобразования строки: {line.strip()}")
        return matrix

    def draw_matrix(self, matrix):
        """Вывод матрицы в красивом формате."""
        for row in matrix:
            row_str = " ".join(f"{elem:8.3f}" for elem in row)
            print(row_str)
        print()

    def create_single_matrix(self, matrix):
        """Создание расширенной матрицы для нахождения обратной."""
        size = len(matrix)
        identity = [[1 if i == j else 0 for j in range(size)] for i in range(size)]
        extended_matrix = [matrix[i] + identity[i] for i in range(size)]
        return extended_matrix


class ElementaryTransformations:
    def __init__(self, matrix) -> None:
        self.matrix = matrix

    def validate_matrix(self):
        """Проверка матрицы на правильность."""
        row_lengths = [len(row) for row in self.matrix]
        if len(set(row_lengths)) != 1:
            raise ValueError("Матрица имеет строки разной длины.")

    def start(self):
        """Основной алгоритм нахождения обратной матрицы."""
        self.validate_matrix()
        self.draw_matrix(self.matrix)

        n = len(self.matrix)

        # Прямой ход
        for i in range(n):
            if self.matrix[i][i] == 0:
                # Замена строк, если ведущий элемент равен нулю
                for k in range(i + 1, n):
                    if self.matrix[k][i] != 0:
                        self.matrix[i], self.matrix[k] = self.matrix[k], self.matrix[i]
                        print(f"Перестановка строк {i} и {k}:")
                        self.draw_matrix(self.matrix)
                        break
                else:
                    raise ValueError(f"Нулевой ведущий элемент в строке {i}. Перестановка невозможна.")

            # Нормализация текущей строки
            divisor = self.matrix[i][i]
            print(f"Нормализация строки {i}, делим на {divisor:.3f}:")
            self.divide_row(i, divisor)
            self.draw_matrix(self.matrix)

            # Обнуление элементов ниже ведущего
            for j in range(i + 1, n):
                if self.matrix[j][i] != 0:
                    multiplier = -self.matrix[j][i]
                    print(f"Обнуляем элемент в строке {j}, столбце {i}, используя строку {i}:")
                    self.add_rows(i, j, multiplier)
                    self.draw_matrix(self.matrix)

        # Обратный ход
        for i in range(n - 1, -1, -1):
            for j in range(i - 1, -1, -1):
                if self.matrix[j][i] != 0:
                    multiplier = -self.matrix[j][i]
                    print(f"Обнуляем элемент в строке {j}, столбце {i}, используя строку {i}:")
                    self.add_rows(i, j, multiplier)
                    self.draw_matrix(self.matrix)

        print("Обратная матрица:")
        self.print_inverse_matrix()

    def divide_row(self, row, divisor):
        """Деление строки на число."""
        if divisor == 0:
            raise ValueError("Деление на ноль невозможно.")
        self.matrix[row] = [elem / divisor for elem in self.matrix[row]]

    def add_rows(self, source_row, target_row, multiplier):
        """Прибавление строки source_row к строке target_row с коэффициентом multiplier."""
        self.matrix[target_row] = [
            target_elem + multiplier * source_elem
            for target_elem, source_elem in zip(self.matrix[target_row], self.matrix[source_row])
        ]

    def draw_matrix(self, matrix=None):
        """Вывод текущей матрицы."""
        matrix = matrix or self.matrix
        for row in matrix:
            print(" ".join(f"{elem:8.3f}" for elem in row))
        print()

    def print_inverse_matrix(self):
        """Выводит обратную матрицу из расширенной матрицы."""
        n = len(self.matrix)
        for row in self.matrix:
            print(" ".join(f"{elem:8.3f}" for elem in row[n:]))
        print()


# Пример использования
if __name__ == "__main__":
    data = Data("input")
    matrix = data.return_matrix()
    if matrix:
        extended_matrix = data.create_single_matrix(matrix)
        print("Начальная расширенная матрица:")
        data.draw_matrix(extended_matrix)

        transformations = ElementaryTransformations(extended_matrix)
        try:
            transformations.start()
        except ValueError as e:
            print(f"Ошибка: {e}")
