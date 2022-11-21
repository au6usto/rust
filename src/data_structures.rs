use rand::Rng;

fn main() {
   //Unsigned integer u8, u16, u32, u64, u128, usize
   //Signed integer i8, i16, i32, i64, i128, isize
   println!("Max u32 : {}", u8::MAX);
   println!("Max u32 : {}", u16::MAX);
   println!("Max u32 : {}", u32::MAX);
   println!("Max u64 : {}", u64::MAX);
   println!("Max usize : {}", usize::MAX);

   println!("Max i32 : {}", i8::MAX);
   println!("Max i32 : {}", i16::MAX);
   println!("Max i32 : {}", i32::MAX);
   println!("Max i64 : {}", i64::MAX);
   println!("Max isize : {}", isize::MAX);

   println!("Max i32 : {}", f32::MAX);
   println!("Max i64 : {}", f64::MAX);

   let is_true: bool = true;
   let my_grade = 'A';

   let random_num: i32 = rand::thread_rng().gen_range(1..101);
   println!("Random: {}", random_num);
}