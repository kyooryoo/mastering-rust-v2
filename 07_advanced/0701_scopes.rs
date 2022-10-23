fn main() {
    let mut b = 4;
    {
        // 父作用域下，b的引用继续有效
        let mut a = 34 + b;
        a += 1;
    }
    // 子作用域已经关闭，a的引用不再有效
    // error[E0425]: cannot find value `a` in this scope
    b = a;
}