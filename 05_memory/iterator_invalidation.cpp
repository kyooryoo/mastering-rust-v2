// % g++ -std=c++11 -o main iterator_invalidation.cpp && ./main
#include <iostream>
#include <vector>

int main() {
    std::vector <int> v{5, 10, 15, 20, 25};
    for (auto it=v.begin(); it!=v.end(); it++)
        if ((*it) == 5)
            v.push_back(-1);
    for (auto it=v.begin(); it!=v.end(); it++) 
        std::cout << (*it) << " ";
    return 0;
}