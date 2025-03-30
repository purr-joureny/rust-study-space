// 从标准库中引用 输入/输出 (I/O) 功能
// std 标准库
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");

    println!("please input your guess.");

    // 创建变量来存储用户输入的值 (可变性变量)
    // ::new 中的 :: 语法表明 new 是一个 String 类型的关联函数(associated function)
    // 关联函数是针对某个类型实现的函数

    // new 是作为创建某种类型新值的常用函数名
    let mut guess = String::new();
    // 调用 io 库中的函数 stdin()
    // 如果开头没有使用 use std::io 引入 io 库(I/O) 输入输出
    // 仍可以通过把函数调用写成 std::io::stdin() 来使用该函数
    // stdin 函数实例返回一个 std::io::Stdin 的实例
    // 这是一种代表终端标准输入的类型
    io::stdin()
        // read_line() 会将用户输入附加到传递给它的字符串中
        // 不过也会返回一个 Result(枚举类型) 的值,
        // 枚举类型变量的值可以是多中可能状态中的一个
        // 我们把每种可能的状态称为一种 枚举成员(variant)
        .read_line(&mut guess) // & 表示这个参数是一个引用(reference)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
    random();
}

fn random() {
    println!("Guess the number!");
    // 生成随机数
    // gen_range 获取一个范围表达式 (range expression) 作为参数
    // 这里使用的这类范围表达式使用了 start..=end 形式
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is : {secret_number}");

    println!("Please input your guess.");
    // 输入数字
    let mut guess = String::new();

    io::stdin()
        // 记录
        .read_line(&mut guess)
        .expect("Failed to read line");
    // 输出
    println!("You guessed: {guess}");
}
