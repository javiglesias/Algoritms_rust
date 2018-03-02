use std::io;
mod lib;
fn blub(disorder:&mut Vec<i32>) -> bool{
    let mut done:bool = false;
    let mut n = disorder.len();
    n-=1;
    while !done && n>0 {
        //println!("breaking order");
        let mut i = 0;
        while i < disorder.len() {
            if i+1 == 10 {
                break;
            }
            //println!("processing {}: {}",i,disorder[i]);
            if disorder[i] < disorder[i+1] {
                let temp = disorder[i];
                disorder[i] = disorder[i+1];
                disorder[i+1] = temp;
                i+=1;
            }
            else {
                i+=1;
            }
        }
        n-=1;
        if n==0 {
            done = true;
            //println!("{:?}",disorder );
        }
    }
    return true;
}
fn main() {
    println!("ORDENACION");
    let mut disorder = vec![10,4,2,5,6,7,3,9,1,8];
    println!("Pre-Burbuja: {:?}",disorder);
    if blub(&mut disorder) {
        println!("Post-Burbuja: {:?}",disorder);
    } 
}