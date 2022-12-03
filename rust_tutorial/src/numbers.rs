fn main() {
   const ONE_MIL: u32 = 1_000_000;
   const PI: f32 = 3.14159;
   let age = "47";
   let mut age: u32 = age.trim().parse()
   .expect("Age wasn't assinged a numbre");
   age = age + 1;
   println!("I'm {} and I want ${}", age, ONE_MIL);
   println!("PI = {}", PI);
}