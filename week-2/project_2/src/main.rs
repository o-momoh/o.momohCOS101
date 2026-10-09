fn main() {
   let t:f64 = 450000.00;
   let m:f64 = 1500000.00;
   let h:f64 = 750000.00;
   let d:f64 = 2850000.00;
   let a:f64 = 250000.00;
   let qty:f64 = 10.00;

   let sum = (2.0*t) + m + (3.0*h) + (3.0*d) + a;
   let avg = sum/qty;
   println!("Sum = {}",qty); 
   println!("Average = {}", avg);
}

