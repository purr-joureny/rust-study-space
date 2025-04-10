fn main() {
    // 变量与可变性
    // 当 let x = 5; 变量是不可变的
    // 添加 mut 后变量为可变
    let mut x = 5;
    println!("The value x is {x}");
    x = 6;
    println!("The value x is {x}");

    // 常量
    // 与变量不一样 它是一个不可变变量
    // 常量是绑定到一个名称的不允许改变的值
    // 不允许对常量使用 mut
    // 常量的关键字为 const, 并且必须标明值的类型
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
    // 隐藏
    // 我们定义一个与之前变量同名的新变量
    // Rust 中是为第一个变量被第二个变量所隐藏(Shadowing)了
    // 可以用相同的名称来隐藏一个变量, 以及重复使用 let 关键字
    // 隐藏 与 mut 的区别
    // 1. 如果没有使用 let 关键字就会导致编译错误, 通过使用 let 关键字我们可以对这个值进行计算,不过计算后的变量仍然是不可变的
    // 2. mut 与 隐藏另一个区别就是, 当再次使用 let 时, 实际上是创建了一个新的变量
    let y = 1;

    let y = y + 1;
    // 作用域
    {
        let y = y * 2;
        println!("The value of y in the inner scope is : {y}");
    }

    println!("The value of y is {y}");
    // 变量的可变性 mut 不能改变变量的类型
    // 但是隐藏可以 let
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces is {spaces}");
}
