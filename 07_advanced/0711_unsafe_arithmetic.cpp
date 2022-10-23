// % g++ -std=c++11 -o main 0710_unsafe_arithmetic.cpp && ./main 
// 以下程序返回 4294967295% 因为编译器自动转换了数据类型且没有提示数据溢出

#include <iostream>
int main(int argc, const char * argv[]) {
    uint foo = 5;
    int bar = 6;
    // 这里结果是 -1 但由于数据类型自动转换为无符号整型，数据向下溢出至最大值
    auto difference = foo - bar;
    std::cout << difference;
    return 0;
}