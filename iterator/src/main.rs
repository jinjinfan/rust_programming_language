
fn main() {
    let v1 = vec![1,2,3];
    for i in v1.iter(){
        println!("Got: {}", i);
    }
    // Consuming adapter
    let total : u32= v1.iter().sum();
    assert_eq!(total, 6);
    // iterator adapter
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("V2 after map: {:?}", v2);
}
