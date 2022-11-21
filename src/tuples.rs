fn main() {
   let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
   println!("Age: {}", my_tuple.0);
   println!("Name: {}", my_tuple.1);
   println!("Earnings: {}", my_tuple.2);

   let(age, name, earnings) = my_tuple;
   println!("Age: {}", age);
   println!("Name: {}", name);
   println!("Earnings: {}", earnings);
}