
# HashSet

1. 下标与项
```rust
    use std::collections::HashSet;

fn main() {
    let mut a = HashSet::new();
    a.insert(10);
    a.insert(2);

    let x:Vec<(usize,usize)> = a.iter().enumerate().map(|(x,y)| (x,*y)).collect();
    println!("{:?}",x);
}
```