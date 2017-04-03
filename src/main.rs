use std::io;


fn main() {
    let num = get_integer();
    let array = build_array(32);
    let primality_array = create_primality_array(32);
    println!("{:?}", primality_array);
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

fn create_primality_array(limit: i32) -> Vec<bool>{
    let mut primality_array = Vec::new();
    let mut do_thinger = 1..limit;
    for i in &mut do_thinger {
        primality_array.push(true)
    }
    return primality_array;
}

