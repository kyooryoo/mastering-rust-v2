// % g++ -std=c++11 -o main 03_iterator_invalidation.cpp && ./main
// 迭代一个整数向量的同时向其推送值，造成迭代器指向无效垃圾地址
#include <iostream>
#include <vector>

int main() {
    std::vector <int> v{5, 10, 15, 20, 25};
    for (auto it=v.begin(); it!=v.end(); it++)
        if ((*it) == 5) {
            v.push_back(-1);
            std::cout << "pushed -1\n";
            std::cout << "length of v:" << v.size() << std::endl;
        } else {
            std::cout << (*it) << " is not 5: -1 is not pushed!\n";
            std::cout << "length of v:" << v.size() << std::endl;
        }
    for (auto it=v.begin(); it!=v.end(); it++) 
        std::cout << (*it) << " ";
    return 0;
}