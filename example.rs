fn main(){
 let s = [[5,6,7],[8,9,10],[21,15,32]];
 let mut is_found = false;
 let target_value_in_1st_row = 5;
 'outer:for i in 0..=2{
   let mut j = 0;
   let result = loop {
   	if j>2{
	  break 'outer; 	
   	}
   	if s[i][j] == target_value_in_1st_row {
   	  is_found = true;
   	  break is_found;
   	}
   	j += 1;
   };
  if result{
   println!("{:?}",result); 
   break;
  } 
 }
 if !is_found{
 print!("{}",is_found);
 }
}
