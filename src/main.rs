use std::io;
// 引用了std库中的io库，其中use 类似其它语言中的import
// 默认情况下，rust会预先即将少量的标准库引入到每个作用域这些项叫prelude（英文意：序曲）
//当使用的类型不在prelude中时，就需要使用use来显式的引入
fn main() {
    println!("guess the number");
    println!("please input your guess.");
    let mut guess=String::new();
    //let 申明了一个变量 其中mut表示这个变量是可变的
    io::stdin().read_line(&mut guess).expect("failed to readline");
    println!("you guess:{}",guess)
}
