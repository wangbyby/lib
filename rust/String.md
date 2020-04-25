# String处理

1. 两个String拼接

2. concat与join
``concat是直接合并``
``join是中间插入项``

```rust
let a1 = vec!["1","2","3","4"];
let res1 = a1.concat();

let a2 = vec!["1","world"];
let res2 = a2.join("the ");
```
