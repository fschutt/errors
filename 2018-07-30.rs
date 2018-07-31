//! This error is basically a reminder that .filter() is inclusive - 
//! the predicate must return **true** on the items you want to **keep**.
//!
//! The mistake was assuming the opposite:

fn main() {
    let a = (0..5).filter(|x| x % 2 == 0).collect::<Vec<usize>>();
    
    println!("{:?}", a); // [0, 2, 4]
}
