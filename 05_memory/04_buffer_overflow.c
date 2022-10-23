// % gcc -o main 04_buffer_overflow.c && ./main
// 在最新的GCC编译器中已经可以识别这里的缓冲区溢出错误

int main() {
    char buf[3];
    buf[0] = 'a';
    buf[1] = 'b';
    buf[2] = 'c';
    buf[3] = 'd';
}