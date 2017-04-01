use std::io;


fn main() {
    let num = get_integer();
    let array = build_array(32);
}

fn build_array(x: i32) -> Vec<i32>{
    let mut array: Vec<i32> = Vec::with_capacity(x as usize);
    for num in 0..x {
        array.push(num);
    }
    return array;
}


fn get_integer() -> i32 {

    println!("Please input the number.");

    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("Failed to read input line.");

    let converted_num = num
        .trim()
        .parse()
        .expect("Wanted a number.");

    return converted_num;
}
