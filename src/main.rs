//use std::io;
//use rand::Rng;
//use std::io::{Write, BufReader, BufRead, ErrorKind};
//use std::fs::File;
//use std::cmp::Ordering;

fn main() {

    references();

}

fn references() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");

    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");

    println!("{}", st2);
}

//fn tuples() {
//    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
//    println!("Age: {}", my_tuple.0);
//    println!("Name: {}", my_tuple.1);
//    println!("Earnings: {}", my_tuple.2);
//
//    let(age, name, earnings) = my_tuple;
//    println!("Age: {}", age);
//    println!("Name: {}", name);
//    println!("Earnings: {}", earnings);
//}

//fn iterators() {
//    let arr_2 = [1,2,3,4,5,6,7,8,9];
//    let mut loop_idx = 6;
//
//    for val in arr_2.iter() {
//        println!("Val: {}", val);
//    }
//}
//
//fn arrays() {
//    let arr_1 = [1, 2, 3, 4];
//    println!("1st: {}", arr_1[0]);
//    println!("Length: {}", arr_1.len());
//
//    let mut loop_idx = 0;
//
//    loop {
//        if arr_1[loop_idx] % 2 == 0 {
//            loop_idx += 1;
//            continue;
//        }
//        if arr_1[loop_idx] == 3 {
//            break;
//        }
//        println!("Val : {}", arr_1[loop_idx]);
//        loop_idx +=1;
//    }
//}
//
//fn condition() {
//    let age: i32 = 9;
//    if (age >= 1) && (age <= 18) {
//        println!("Important Birthday");
//    } else if (age == 21) || (age == 50) {
//        println!("Important birth")
//    } else if age >= 25 {
//        println!("Important")
//    } else {
//        println!("Not important Birthday")
//    }
//
//    let age2: i32 = 8;
//    match age2 {
//        1..=18 => println!("Important Birthday"),
//        21 | 50 => println!("Important Birthday"),
//        65..= i32::MAX => println!("Important Birthday"),
//        _ => println!("Not an Important Birthday")
//    };
//
//    let my_age = 18;
//    let voting_age = 18;
//    match my_age.cmp(&voting_age) {
//        Ordering::Less => println!("Can't vote"),
//        Ordering::Greater => println!("Can vote"),
//        Ordering::Equal => println!("Can vote right now"),
//    }
//}
//
//fn data_structures() {
//    //Unsigned integer u8, u16, u32, u64, u128, usize
//    //Signed integer i8, i16, i32, i64, i128, isize
//    println!("Max u32 : {}", u8::MAX);
//    println!("Max u32 : {}", u16::MAX);
//    println!("Max u32 : {}", u32::MAX);
//    println!("Max u64 : {}", u64::MAX);
//    println!("Max usize : {}", usize::MAX);
//
//    println!("Max i32 : {}", i8::MAX);
//    println!("Max i32 : {}", i16::MAX);
//    println!("Max i32 : {}", i32::MAX);
//    println!("Max i64 : {}", i64::MAX);
//    println!("Max isize : {}", isize::MAX);
//
//    println!("Max i32 : {}", f32::MAX);
//    println!("Max i64 : {}", f64::MAX);
//
//    let is_true: bool = true;
//    let my_grade = 'A';
//
//    let random_num: i32 = rand::thread_rng().gen_range(1..101);
//    println!("Random: {}", random_num);
//}
//
//fn numbers() {
//    const ONE_MIL: u32 = 1_000_000;
//    const PI: f32 = 3.14159;
//    let age = "47";
//    let mut age: u32 = age.trim().parse()
//    .expect("Age wasn't assinged a numbre");
//    age = age + 1;
//    println!("I'm {} and I want ${}", age, ONE_MIL);
//    println!("PI = {}", PI);
//}