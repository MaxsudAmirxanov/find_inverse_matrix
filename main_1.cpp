#include <iostream>
#include <fstream>
#include <string>     // для std::getline
using namespace std;
int main()
{
    string line;
 
    ifstream in("input.txt"); // окрываем файл для чтения
    if (in.is_open())
    {
        while (std::getline(in, line))
        {
            std::cout << line << std::endl;
        }
        
    }
    in.close();     // закрываем файл
}