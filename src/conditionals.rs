use std::cmp::Ordering;

fn main() {
   let age: i32 = 9;
   if (age >= 1) && (age <= 18) {
       println!("Important Birthday");
   } else if (age == 21) || (age == 50) {
       println!("Important birth")
   } else if age >= 25 {
       println!("Important")
   } else {
       println!("Not important Birthday")
   }

   let age2: i32 = 8;
   match age2 {
       1..=18 => println!("Important Birthday"),
       21 | 50 => println!("Important Birthday"),
       65..= i32::MAX => println!("Important Birthday"),
       _ => println!("Not an Important Birthday")
   };

   let my_age = 18;
   let voting_age = 18;
   match my_age.cmp(&voting_age) {
       Ordering::Less => println!("Can't vote"),
       Ordering::Greater => println!("Can vote"),
       Ordering::Equal => println!("Can vote right now"),
   }
}