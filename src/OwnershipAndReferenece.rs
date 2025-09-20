fn main() {
    let mut v: Vec<i32> = vec![1,2,3];
    let num: &i32 = &v[2];
    
    println!("Third ele is {}", *num);
    v.push(4);
    
    let num: i32 = v[3];
    println!("Forth ele is {}", num);
    
    let num: &mut i32 = &mut v[2];
    let num2: &i32 = &*num;
    
    println!("*num: {}, *num2: {}", *num, *num2);
}