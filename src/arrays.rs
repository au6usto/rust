fn main() {
   let arr_1 = [1, 2, 3, 4];
   println!("1st: {}", arr_1[0]);
   println!("Length: {}", arr_1.len());

   let mut loop_idx = 0;

   loop {
       if arr_1[loop_idx] % 2 == 0 {
           loop_idx += 1;
           continue;
       }
       if arr_1[loop_idx] == 3 {
           break;
       }
       println!("Val : {}", arr_1[loop_idx]);
       loop_idx +=1;
   }
}