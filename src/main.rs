fn main() {
    // 创建一个可变的整型向量
    let mut v: Vec<i32> = vec![1,2,3];

    // 获取第三个元素的不可变引用
    let num: &i32 = &v[2];
    println!("Third ele is {}", *num);

    // 向向量添加新元素，可能导致重新分配
    v.push(4);

    // 获取第四个元素的值（此时不能再用之前的引用）
    let num: i32 = v[3];
    println!("Forth ele is {}", num);

    // 获取第三个元素的可变引用
    let num: &mut i32 = &mut v[2];

    // 通过解引用获得不可变引用
    let num2: &i32 = &*num;

    // 打印可变引用和不可变引用的值
    println!("*num: {}, *num2: {}", *num, *num2);

    let mut n = 5;
    incr(&mut n);

    println!("n: {}", n);
}

fn incr(n: &mut i32) {
    *n += 1;
}
