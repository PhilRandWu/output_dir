use std::borrow::Cow;

use std::fs::File;
use std::option;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/* 递归 */
pub fn recur(n: i32) -> i32 {
    // 终止条件
    if n == 1 {
        return 1;
    }
    // 递：递归调用
    let res = recur(n - 1);
    // 归：返回结果
    n + res
}

/* 尾递归 */
pub fn tail_recur(n: i32, res: i32) -> i32 {
    // 终止条件
    if n == 0 {
        return res;
    }
    // 尾递归调用
    tail_recur(n - 1, res + n)
}


/* 斐波那契数列：递归 */
pub fn fib(n: i32) -> i32 {
    // 终止条件 f(1) = 0, f(2) = 1
    if n == 1 || n == 2 {
        return n - 1;
    }
    // 递归调用 f(n) = f(n-1) + f(n-2)
    fib(n - 1) + fib(n - 2)
}
