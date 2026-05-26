use custom_hashmap::hashmap::HashMap;
fn main() {
    let mut map = HashMap::<i32, i32>::new();
    for i in 0..100 {
        map.insert(i, i);
    }

    for i in 0..100 {
        println!("{}", map.get(&i).unwrap());
    }
}
