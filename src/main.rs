
// let is_even = |val:i32|-> bool {
//     if (val % 2 == 0){
//         return true;
//     }
//     else {
//         return false;
//     }
// };
fn main(){
    let mut sum=0;
    let add=|n1:i32 ,n2:i32| n1+n2;
    let numbers=(3..12)
        .inspect(|n| println!("before filter  = {}",n))
        .filter(|n| n%2 == 0 )
        .inspect(|n| println!("after filter n = {}",n));
n
    for n in numbers{
        sum=add(sum,n);
    }
    println!("sum = {}",sum);
}