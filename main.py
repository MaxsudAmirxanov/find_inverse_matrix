class Data:
    def __init__(self, file_name) -> None:
        self.file_name = file_name

    def get_data(self):
        # d = [1]
        with open(f'{self.file_name}.txt', 'r+') as f:
            # print(f.readlines())
            d = f.readlines()
            print(d)
        return d
    def return_matrix(self):
        d = [[1,1,1,1], [1,1,1,1]]
        matrix = []
        data = self.get_data()
        print(111)
        print(data)
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
        print(list)


class Matrix:
    def __init__(self) -> None:
        pass
    
    def find_inverse(self):
        pass
    def create_zero_matrix():
         

data = Data('input')

data.return_matrix()