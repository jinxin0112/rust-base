fn main() {
    let f = foo(4, 5);
    println!("f={}", f);

    bar();

    let fib_number = fib(30);
    println!("fib_number={}", fib_number);
}

fn foo(x: i32, y: i32) -> i32 {
    let z = { x + y };
    println!("z={}", z);
    z
}

fn bar() {
    let mut number = 5;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("退出了");

    let arr = [1, 2, 3, 4];
    for ele in arr.iter() {
        println!("{}", ele)
    }
    for ele in (1..4).rev() {
        // (1..4) 表示生成一个 1到3 的序列
        println!("{}", ele)
    }
}

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    let mut current_value = 1;
    let mut pre_value = 0;

    for _i in 2..(n + 1) {
        let sum = current_value + pre_value;
        pre_value = current_value;
        current_value = sum;
    }

    return current_value;
}
