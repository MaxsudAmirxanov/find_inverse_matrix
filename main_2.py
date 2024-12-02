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
            row = []
            try:
                elements = line.strip().split()
                for elem in elements:
                    row.append(int(elem))
                matrix.append(row)
            except ValueError:
                print(f"Ошибка преобразования строки: {line.strip()}")
        return matrix

    def draw_matrix(self, matrix):
        """Вывод матрицы в красивом формате."""
        for row in matrix:
            row_str = ""
            for elem in row:
                row_str += f"{elem:8.3f} "
            print(row_str)
        print()
    def create_single_matrix(self, matrix):
        """Создание расширенной матрицы для нахождения обратной."""
        size = len(matrix)
        identity = []
        for i in range(size):
            row = []
            for j in range(size):
                if i == j:
                    row.append(1)
                else:
                    row.append(0)
            identity.append(row)

        extended_matrix = []
        for i in range(size):
            extended_row = []
            for elem in matrix[i]:
                extended_row.append(elem)
            for elem in identity[i]:
                extended_row.append(elem)
            extended_matrix.append(extended_row)

        return extended_matrix


# class Matrix:
#     def __init__(self, matrix) -> None:
#         self.matrix = matrix

#     def create_single_matrix(self):
#         """Создание расширенной матрицы для нахождения обратной."""
#         size = len(self.matrix)
#         identity = []
#         for i in range(size):
#             row = []
#             for j in range(size):
#                 if i == j:
#                     row.append(1)
#                 else:
#                     row.append(0)
#             identity.append(row)

#         extended_matrix = []
#         for i in range(size):
#             extended_row = []
#             for elem in self.matrix[i]:
#                 extended_row.append(elem)
#             for elem in identity[i]:
#                 extended_row.append(elem)
#             extended_matrix.append(extended_row)

#         return extended_matrix


class ElementaryTransformations(Data):
    def __init__(self, matrix) -> None:
        self.matrix = matrix

    def validate_matrix(self, matrix):
        row_lengths = [len(row) for row in matrix]
        if len(set(row_lengths)) != 1:
            print(row_lengths)
            raise ValueError("Матрица имеет строки разной длины.")
        
        # for i in range(self.matrix):
        #     if len(i) != 

    def start(self):
        self.draw_matrix(self.matrix)
        # print('')
        self.validate_matrix(self.matrix)
        for i in range(len(self.matrix)):
            print(i)
            # Проверка на нулевой ведущий элемент
            if self.matrix[i][i] == 0:
                print(f"Нулевой ведущий элемент в строке {i}. Ищем строку для замены...")
                for k in range(i + 1, len(self.matrix)):
                    if self.matrix[k][i] != 0:  # Ищем ненулевой элемент в текущем столбце
                        print(f"Перестановка строк {i} и {k}.")
                        self.matrix[i], self.matrix[k] = self.matrix[k], self.matrix[i]  # Перестановка строк
                        self.draw_matrix(self.matrix)
                        break
                else:
                    raise ValueError(f"Нулевой ведущий элемент в строке {i}. Перестановка строк невозможна.")
            
            print(f'Нахождение единички в диагонали. Делим строку {i} на {self.matrix[i][i]}')
            self.division_line(i, self.matrix[i][i])
            self.draw_matrix(self.matrix)

            for j in range(i + 1, len(self.matrix)):
                # print(j)
                index = -(self.matrix[j][i])
                print(f'Обнуляем элемент в строке {j}, столбце {i} с помощью строки {i}')
                self.adding_line_2(i, j, index)
                self.draw_matrix(self.matrix)

        print("\nНачинаем обратный проход для приведения матрицы к диагональному виду:")
        for i in range(len(self.matrix) - 1, -1, -1):
            print(f"Обратный проход: Работа с {i}-й строкой")
            for j in range(i - 1, -1, -1):
                if self.matrix[j][i] != 0:
                    multiplier = -self.matrix[j][i]
                    print(f"Обнуляем элемент в строке {j}, столбце {i} с помощью строки {i}:")
                    self.adding_line_2(i, j, multiplier)
                    self.draw_matrix(self.matrix)

        self.draw_matrix(self.matrix)

        

    def division_line(self, line_number, divisor):
        """Деление строки на число."""
        if divisor == 0:
            raise ValueError("Деление на ноль невозможно.")
        for i in range(len(self.matrix[line_number])):
            self.matrix[line_number][i] = self.matrix[line_number][i] / divisor

    def adding_line_2(self, source_row, target_row, multiplier):
        """Прибавление строки source_row к строке target_row с коэффициентом multiplier."""
        for i in range(len(self.matrix[source_row])):
            self.matrix[target_row][i] = self.matrix[target_row][i] + multiplier * self.matrix[source_row][i]


# Пример использования
data = Data("input")
# m = Matrix(data.return_matrix())
matrix = data.create_single_matrix(data.return_matrix())
# print(matrix)
import numpy as np

# Генерация матрицы 1000x1000 со случайными числами от -100 до 100
# matrix_1000x1000 = np.random.randint(-100, 101, size=(1000, 1000))
# print(matrix_1000x1000)
# print(111)


if matrix:
    
    # print(m.create_single_matrix())
    # for index, i in enumerate(matrix):
    #     matrix[index] = matrix[index] + m.create_single_matrix()[index]
    print(matrix)
    transformations = ElementaryTransformations(matrix)
    transformations.start()
