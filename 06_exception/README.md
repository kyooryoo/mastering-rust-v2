异常
* 可恢复异常：文件未找到或者数值解析错误 - 程序交互时可预期发生
* 不可恢复异常：索引越界或者除以零 - 违反约定或者程序常量
* 致命异常：内存不足或者堆栈溢出 - 程序立即终止

错误处理
* C 语言使用返回代码方式，需要程序员手动捕获
* Java 使用抛出异常方式，但开销较大且不确定是否可抛出
* Rust 使用返回错误类型的方式，开销低且更安全
