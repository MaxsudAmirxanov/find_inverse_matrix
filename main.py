class Data:
    def __init__(self, file_name) -> None:
        self.file_name = file_name

    def get_data(self):
        # d = [1]
        with open(f'{self.file_name}.txt', 'r+') as f:
            # print(f.readlines())
            d = f.readlines()
            # print(d)
        return d
    def return_matrix(self):
        d = [[1,1,1,1], [1,1,1,1]]
        matrix = []
        data = self.get_data()
        # print(111)
        # print(data)
        sublist = []
        list = []
        for i in data:
            for j in i:
                # print(j)
                try:
                    int(j)
                    sublist.append(int(j))

                    
                except:
                    pass
            list.append(sublist)
            sublist = []
        # print(list)
        return list
    
    def draw_matrix(self, matrix):
        matrix_line_str = ''
        for i in matrix:
            print(i)

            




class Matrix:
    def __init__(self, matrix) -> None:
        self.matrix = matrix
    
    def find_inverse(self):
        pass
    def create_single_matrix(self):
        l = len(self.matrix)
        r = len (self.matrix[1])
        # print(r)
        # print(l)
        new_matrix = []
        for index, line in enumerate(self.matrix):
            single_matrix = []
            for j in range(0, len(self.matrix[1])):
                if index == j:
                    single_matrix.append(1)
                else:
                    single_matrix.append(0)
                    
            new_matrix.append(single_matrix)
        # print('new_matrix', new_matrix)
        # return new_matrix
         
        for index, line in enumerate(self.matrix):
            # print('matrix', self.matrix)
            # print('line new_matrix', line, new_matrix)
            new_matrix[index] = line + new_matrix[index]



        return new_matrix



class ElementaryTransformations(Data):
    def __init__(self, matrix) -> None:
        self.matrix = matrix

    def start(self):
        self.draw_matrix(self.matrix)
        print('')
        for i in range(len(self.matrix)):

            
            print('Нахождение единички в диагонали')
            print(f'Деление всех элементов {i} строки на {self.matrix[i][i]}')



            self.division_line(i, self.matrix[i][i])
            self.draw_matrix(self.matrix)

            

            for j in range(i+1, len(self.matrix)):
                index = -(self.matrix[j][i]) 
                
                # print(f'прибавление к всем элементов {j} строка на {index}')
                print(f'умнажаем строку {i} на {index} и прибовлаем к строке {j}')
                print(i, j, 'i j')
                self.adding_line_2( i, j, index)
                self.draw_matrix(self.matrix)
            
        
        self.draw_matrix(self.matrix)
        print(1111)
        print(self.matrix)

        print("\nНачинаем обратный проход для приведения матрицы к диагональному виду:")
        for i in range(len(self.matrix) - 1, -1, -1):  # Идём с конца к началу
            print(f"Обратный проход: Работа с {i}-й строкой")

            for j in range(i - 1, -1, -1):  # Обнуляем элементы выше диагонали
                if self.matrix[j][i] != 0:
                    multiplier = -self.matrix[j][i]
                    print(f"Обнуляем элемент в строке {j}, столбце {i} с помощью строки {i}:")
                    self.adding_line_2(i, j, multiplier)
                    self.draw_matrix(self.matrix)

        # for i in reversed(range(len(self.matrix))):
        #     print('Нахождение единички в диагонали')
        #     print(f'Деление всех элементов {i} строки на {self.matrix[i][i]}')



        #     self.division_line(i, self.matrix[i][i])
        #     self.draw_matrix(self.matrix)

            

        #     for j in reversed(range(i-1, len(self.matrix))):
        #         index = -(self.matrix[j][i]) 
                
        #         # print(f'прибавление к всем элементов {j} строка на {index}')
        #         print(f'умнажаем строку {i} на {index} и прибовлаем к строке {j}')
        #         print(i, j, 'i j')
        #         self.adding_line_2( i, j, index)
        #         self.draw_matrix(self.matrix)
            
        
        self.draw_matrix(self.matrix)
        print(1111)
        print(self.matrix)

    def multiply_line(self, line_number, number ):
        'умножение какойто строки '
        for index, integer in enumerate(self.matrix[line_number]):
            self.matrix[line_number][index] = self.matrix[line_number][index] * number
            if self.matrix[line_number][index] * number in [-0.0, -0]:
                self.matrix[line_number][index] = 0
    

    def division_line(self, line_number, number ):
        'деление какойто строки '
        for index, integer in enumerate(self.matrix[line_number]):
            self.matrix[line_number][index] = self.matrix[line_number][index] / number
            if self.matrix[line_number][index] / number in [-0.0, -0] :
                self.matrix[line_number][index] = 0
    
        # print(self.matrix)

    def adding_line(self, line_number, number, over):
        for index, integer in enumerate(self.matrix[line_number]):
            # for i in self.matrix[over]:

            self.matrix[line_number][index] = self.matrix[line_number][index]   + number

    def adding_line_2(self, x_from, x_to, number):
        'c x_from прибавить на x_to'
        for index, i in enumerate(self.matrix[x_from]):
            # print(i)
            self.matrix[x_to][index] =  self.matrix[x_to][index] +i*number
            if (self.matrix[x_to][index] +i*number) in [-0.0, -0]:
                self.matrix[x_to][index] = 0
    
    def multiply_line_2(self, x_from, x_to, number):
        'умножаем x_from на x_to'
        for index, i in enumerate(self.matrix[x_from]):
            # print(i)
            self.matrix[x_to][index] =  self.matrix[x_to][index] +i*number
            if (self.matrix[x_to][index] +i*number) in [-0.0, -0]:
                self.matrix[x_to][index] = 0

    def division_line_2(self, x_from, x_to, number):
        'деление x_from / x_to'
        for index, i in enumerate(self.matrix[x_from]):
            # print(i)
            self.matrix[x_to][index] =  self.matrix[x_to][index] +i/number
            if (self.matrix[x_to][index] +i/number) in [-0.0, -0]:
                self.matrix[x_to][index] = 0


print(0.5*-1.0)
data = Data('input')

matrix = Matrix(data.return_matrix())

m = ElementaryTransformations(data.return_matrix())

m.start()
print([1,2,3])
print(0.5*-1.0)

for index, i in enumerate(matrix):
    matrix[index] = i+data.return_matrix()[index]
print(matrix)
# data.draw_matrix(matrix.create_single_matrix())