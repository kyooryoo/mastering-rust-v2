// % gcc -o main 02_uninitialized_reads.c && ./main
// 运行如上命令编译和运行本程序，注意每次结果都不用

#include <stdio.h>
int main() {
    int values[5];
    for (int i = 0; i < 5; i ++) {
        printf("%d ", values[i]);
    }
}