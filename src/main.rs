fn main() {
    println!("Hello, world!");

    let x = 5; // imutable can't not change
    let mut y = 10; //  mutable can't change
    y = 20;
    // x = 30 => error

    // const !== immutable
    const A_B: i32 = 2;

    // cosnt tồn tại xuốt toàn bộ chường trình và phải đc khai báo type từ đầu
    // immutable có thể khai báo lại

    //đảo 1 chuỗi cho trước
    let input = "Huy Van";
    let mut output = String::new();

    for i in input.chars() {
        output.push(i);
    }
    println!("test = {} to {}", input, output);
}
