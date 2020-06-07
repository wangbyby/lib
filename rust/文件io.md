## io
- 标准输入输出
```rust
      
    use std::io;
    use std::io::prelude::*;// use std::io::Write;
    const PROMPT : &'static str= ">> ";
    fn main() {
        
        //read
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut buff = [0;512];
        io::stdin().read(&mut buff).unwrap();

        //write
        io::stdout().write(&input).unwrap();
        io::stdout().write(b"Hello Monkey\n");
        io::stdout().flush().unwrap(); //注意flush
    
        io::stdout().write(PROMPT.as_bytes());
        io::stdout().flush().unwrap();
    }
    
```
- 读写文件

    1. 读文件

```rust
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::io;
    fn main() {
        
        let path = Path::new("hello.txt");
        let display = path.display();
        //打开文件只读模式
        let mut file = match File::open(&path) {
            Err(why) => panic!("can't open {} : {}",display, Error::description(&why)),
            Ok(file) => file,
        };
        let mut string = String::new();
        match file.read_to_string(&mut string){
            Err(why) => panic!("can't open {} : {}",display, Error::description(&why)),
            Ok(_) => println!("{} has {}",display, string),
        }
    }
```
    2. 写文件
```rust
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::io;
    fn main() {
        
        let path = Path::new("hello.txt");
        let display = path.display();
        //只写模式
        let mut file = match File::create(&path) {
            Err(why) => panic!("can't create {} : {}",display, Error::description(&why)),
            Ok(file) => file,
        };
        let mut string = String::new();
        string.push_str("string: &str");
    
        match file.write_all(string.as_bytes()){
            Err(why) => panic!("can't open {} : {}",display, Error::description(&why)),
            Ok(_) => println!("{} has {}",display, string),
        }
    }

```