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

data = Data('input')

matrix = Matrix(data.return_matrix())


# data.return_matrix()
data.draw_matrix(matrix.create_single_matrix())