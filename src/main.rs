fn main(){
    let mut sum :i32=0;
    let add =|n1:i32,n2:i32| n1+n2;
    let numbers=3..10;
    for n in numbers.into_iter(){
        sum=add(sum,n);
    }
    println!("sum = {}", sum);
}