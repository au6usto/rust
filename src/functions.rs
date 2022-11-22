use std::ops::Add;

fn get_sum(x:i32, y: i32) -> i32 {
    x + y
}

fn get_2_values(x: i32) -> (i32, i32) {
    return (x+1, x+2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter()  {
        sum += &val;
    }
    sum
}

fn get_sum_generic<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    println!("{}", get_sum(4, 5));

    let (val_1, val_2) = get_2_values(4);
    println!("Nums: {} {}", val_1, val_2);

    let num_list = vec![1,2,3,4,5];
    println!("Sum of List = {}", sum_list(&num_list));

    //Generic Type
    println!("{}", get_sum_generic(4, 5));
    println!("{}", get_sum_generic(4.3, 5.2));
}