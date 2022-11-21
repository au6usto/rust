fn main() {
    let st3 = String::from("x r t p f h g g x");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();

    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    println!("{:?}", byte_arr1);

    let st6: &str = &st5[00..6];
    println!("String length : {}", st6.len());
    st5.clear();

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;

    for char in st8.bytes() {
        println!("{}", char);
    }
}