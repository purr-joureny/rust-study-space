// crate 是一组 rust 源码文件
// 我们正在构建的项目是一个 二进制 crate
// std 标准库
// Rng 是一个 trait, 它定义了随机数生成器应实现的方法
// 想使用这些方法的话, 此 trait 必须在作用域中
use rand::Rng;
use core::num;
use std::cmp::Ordering;
// 从标准库中引用 输入/输出 (I/O) 功能
use std::io;

fn main() {
    // println!("Guess the number");

    // println!("please input your guess.");

    // // 创建变量来存储用户输入的值 (可变性变量)
    // // ::new 中的 :: 语法表明 new 是一个 String 类型的关联函数(associated function)
    // // 关联函数是针对某个类型实现的函数

    // // new 是作为创建某种类型新值的常用函数名
    // let mut guess = String::new();
    // // 调用 io 库中的函数 stdin()
    // // 如果开头没有使用 use std::io 引入 io 库(I/O) 输入输出
    // // 仍可以通过把函数调用写成 std::io::stdin() 来使用该函数
    // // stdin 函数实例返回一个 std::io::Stdin 的实例
    // // 这是一种代表终端标准输入的类型
    // io::stdin()
    //     // read_line() 会将用户输入附加到传递给它的字符串中
    //     // 不过也会返回一个 Result(枚举类型) 的值,
    //     // 枚举类型变量的值可以是多中可能状态中的一个
    //     // 我们把每种可能的状态称为一种 枚举成员(variant)
    //     .read_line(&mut guess) // & 表示这个参数是一个引用(reference)
    //     .expect("Failed to read line");

    // println!("You guessed {}", guess);
    // random();
    diff_num();
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

fn diff_num() {
    // snip ...
   
    // loop 关键字创建无限循环
    loop {
        println!("请输入..");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // 这里 rust 允许用一个新值来隐藏(shadowing)之前的值
        // trim 方法去除开头和结尾的空白字符
        // 我们必须使用此方法才能将字符串与 u32 比较
        // parse 将字符串转换成其他类型, 我们要告诉 Rust 具体的数字类型
        // 如果 用户输入的不是一个数字 例如: 3%# 这样的话程序就会崩溃
        // 这里 通过 let guess: u32 指定
        // 我们将 expect 调换成 match 语句
        // 以从遇到错误就崩溃转换为处理错误
        // 须知 parse 返回一个 Result 类型, 而 Result 是一个拥有 Ok 或 Err 成员的枚举 
        // 这里的 match 分支执行 用户输入 处理结果为数字时 与 match 第一个模式匹配
        // 返回 Ok 值中的 num 
        // 相反则 匹配第二个 Err 分支
        // _ 是一个通配符, 来匹配所有 Err 的值
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你的数字: {guess}");
        // 数字类型拥有 1 到 100 之间的值
        // 32位数字 i32
        // 32位无符号数字 u32
        // 64位数字 i64
        // rust 默认使用 i32
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
        println!("随机数: {secret_number}");
    
        // 这里将 guess 和 secret_number 通过调用 cmp 来对比
        // match 表达式由分支(arms)构成
        // 一个分支包含一个模式(pattern)和表达式开头的值与分支模式相匹配时应该执行的代码

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            // 在 Equal 添加 break 语句 在用户猜对时退出游戏
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
