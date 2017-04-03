use std::io;


fn main() {
    let num = get_integer();
    let mut primality_array = create_primality_array(num);
    println!("{:?}", primality_array);

    let mut p_list = Vec::new();
    for (i, item) in primality_array.iter().enumerate() {
        println!("{:?}", i+1);
        if item == true {
            p_list.push(i);
            for n in (i*i..num).step_by(i) {
                primality_array[n] = false
            }
        }
    }
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
    let mut do_thinger = 1..(limit + 1);
    //for i in &mut do_thinger {
    for i in 1..limit{
        primality_array.push(false)
    }
    return primality_array;
}
