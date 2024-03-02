
fn main() {
    // // variable x is immutable
    // let x=5;
    // // 变量默认是不可变的，如果需要改变变量的值，需要使用mut关键字
    // let mut y=6;
    // y+=1;
    // println!("Hello, world! x={} y={}", x, y);
    // 变量作用域
    let x=5;
    {
        let y=6;
        println!("x={} y={}", x, y);
    }
    println!("x={}", x);
    // 变量遮蔽
    let x=5;
    

}