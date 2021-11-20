pub fn run() {
    let a1: [u8; 7000000] = [1; 7000000];

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1 is: {:?}", &v1);
    println!("Stack address of v2 is: {:?}", &v2);
    println!("Heap memory address of v1: {}", v1.len());
    println!("Len of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);
}
