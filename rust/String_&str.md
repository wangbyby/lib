# char,String,&str 
  - 转换
    ```rust
        "\r\t\n let ax = helo; ".split_ascii_whitespace();
        let str0  = "hello".to_string();//&str --> String
        let str1 = 'a'.to_string(); // char --> String
        let c : char = 0 as char; //0作为char
        self.ch = self.input.chars().nth(self.read_position).unwrap_or(CHAR0);//得到String的第n个char
        
    
    ``` 
  - Cow 写时复制
    ```rust
    //copy on write
    use std::borrow::Cow;
    fn cow_test<'a> (input :&'a str) -> Cow<'a,str>{
        if input.contains(' ') {
            let mut buf  = String::with_capacity(input.len());
            for i in input.chars() { //这里String是没有 buf[0](按下标取值)操作的
                if i != ' ' {
                    buf.push(i);
                }
            }
            return Cow::Owned(buf);
        }
        Cow::Borrowed(input)
    }   
    ```
  - From,Into泛型的使用
    ```rust
        fn read_str<S: Into<String>>(s: S){
            s.into() //转换为String
        }
    ```

## String处理

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


