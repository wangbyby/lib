- DSL (领域专用语言)
    1. 单个参数
    ```rust
    macro_rules! compute {
        (eval $e:expr) => {
            let val: isize = $e;
            println!("{} = {}",stringify!($e),val);
        };
    }
    fn main() {
        compute!{
            eval 1+2
        }
        compute!{
            eval (2+2)*(1-2)/3
        }
    }
    ```
    2. 多个参数
    ```rust
    macro_rules! compute {
        //单个参数
        (eval $e:expr) => {
            let val: isize = $e;
            println!("{} = {}",stringify!($e),val);
        };
        //多个
        (eval $e:expr, $(eval $es:expr),+) => {
            compute!{eval $e}
            compute!{ $(eval $es),+}
        };
    }
    fn main() {
        
        compute!{
            eval 1+2,
            eval 3+4 //不加逗号
        }

        compute!{
            eval (2+2)*(1-2)/3
        }
    }
    ```
- 全局静态变量-lazy_static!
    ```rust
    #[macro_use]
    extern crate lazy_static;

    use std::collections::HashMap;

    lazy_static! {
        static ref VEC:Vec<u8> = vec![0x18u8, 0x11u8];
        static ref MAP: HashMap<u32, String> = {
            let mut map = HashMap::new();
            map.insert(18, "hury".to_owned());
            map
        };
        static ref PAGE:u32 = mulit(18);
    }

    fn mulit(i: u32) -> u32 {
        i * 2
    }

    fn main() {
        println!("{:?}", *PAGE);
        println!("{:?}", *VEC);
        println!("{:?}", *MAP);
    }
    ```
- 标准输入输出
```rust
      
    use std::io;
    use std::io::prelude::*;// use std::io::Write;
    const PROMPT : &'static str= ">> ";
    fn main() {
        
        //读
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut buff = [0;512];
        io::stdin().read(&mut buff).unwrap();

        //写
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

- rust类型系统
    - 关联类型
    ```rust
    //trait的定义
    trait Graph {
        type N : fmt::Display;
        type E;

        fn has_edge(&self, &Self::N, &Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
    // etc
    }

    //实现
    struct Node;
    struct Edge;
    struct MyGraph;

    impl Graph for MyGraph {
        type N = Node;
        type E = Edge;

        fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
            true
        }

        fn edges(&self, n: &Node) -> Vec<Edge> {
            Vec::new()
        }
    }
    ```
 - trait --> 具体类型

    - 类似go的接口断言
    ```rust
    let tests = vec![
        Tests::new("x"),
        Tests::new("y"),
        Tests::new("foobar"),
    ];
    
    
    for (i,v)  in tests.iter().enumerate() {
        let tmp:& Box<dyn ast::Node> = &program.statements[i];
        
        let b: &ast::LetStatement = match tmp.as_any().downcast_ref::<ast::LetStatement>() {
            Some(b) =>b,
            None =>panic!("&Node is not a let statement"),
        };
        
        test_let_statment_helper(b,&v.expectedIdentifier);
    }

    //结构体定义
    use std::any::Any;
    pub trait Node: std::fmt::Debug {
        fn token_literal(&self) -> String;
        fn as_any(&self) -> &dyn Any;
    }
    
    impl  Node for Program {
    fn token_literal(&self) ->String {
        if self.statements.len()>0 {
            return self.statements[0].token_literal();
        }
        "".to_string()
    }
    fn as_any(&self)->&dyn Any{
        self
    }
}
    ```



    ```rust
    use std::any::Any;
    trait A {
        fn as_any(&self) -> &dyn Any;
    }
    
    struct B;
    impl A for B {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    fn main() {
        let a: Box<dyn A> = Box::new(B);
        // The indirection through `as_any` is because using `downcast_ref`
        // on `Box<A>` *directly* only lets us downcast back to `&A` again.
        // The method ensures we get an `Any` vtable that lets us downcast
        // back to the original, concrete type.
        let b: &B = match a.as_any().downcast_ref::<B>() {
            Some(b) => b,
            None => panic!("&a isn't a B!"),
        };
    }
    ```
