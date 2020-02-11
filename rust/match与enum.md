## match用法
  - @与 a..=b 的用法
    a..=b 表示范围 a<= , <=b
    ```rust
    fn main(){
        enum Message {
            Hello { id: i32 },
        }
        
        let msg = Message::Hello { id: 5 };
        
        match msg {
            //在解析的同时 使用@, 来检测值的范围
            Message::Hello { id: id_variable @ 3..=7 } => {
                println!("Found an id in range: {}", id_variable)
            },
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            },
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            },
        }
    }
    ```
     - 解构结构体

    ```rust
    struct Point {
        x: i64,
        y: i64,
    }

    let point = Point { x: 0, y: 0 };
    match point {
        Point { x: x1, y: y1} => println!("({},{})", x1, y1),
    }
    match point {
        Point { y, .. } => println!("y is {}", y), //忽略某些情况
    }
    ```
     - 现在还不能匹配表达式, 2020-1-28

## 枚举
    ```rust
    #[derive(Debug)]
    enum Operation {
        Move {x: i32, y:i32},
        Jump(u32),
        Attack(i32),
    }

    fn main() {
        // 定义一个移动的操作
        let opt_move = Operation::Move {x: 10, y: 11};
    
        // 定义一个攻击的操作
        let opt_attack = Operation::Attack(100);
    
        // 定义一个跳跃的操作
        let opt_jump = Operation::Jump(3);
        match opt_move {
            Operation::Move{x,..}=> println!("x= {}",x), //注意是两个点.., 而且注意按结构体的字段名x匹配
            _=> println!("do others"),
            
        }
      
    }

    // 执行操作
    fn DoOperation(opt: Operation) {
        println!("Do operation: {:?}", opt);
    }
    
    ```
    ###在枚举中使用use
    ```rust
    enum Status {
        Rich,
        Poor,
    }

    enum Work {
        Civilian,
        Soldier,
    }

    fn main() {
        // 显式地 `use` 各个名称使他们直接可用，而不需要指定它们来自 `Status`。
        use Status::{Poor, Rich};
        // 自动地 `use` `Work` 内部的各个名称。
        use Work::*;
        // `Poor` 等价于 `Status::Poor`。
        let status = Poor;
        // `Civilian` 等价于 `Work::Civilian`。
        let work = Civilian;

        match status {
            // 注意这里没有用完整路径，因为上面显式地使用了 `use`。
            Rich => println!("The rich have lots of money!"),
            Poor => println!("The poor have no money..."),
        }

        match work {
            // 再次注意到没有用完整路径。
            Civilian => println!("Civilians work!"),
            Soldier  => println!("Soldiers fight!"),
        }
    }
    ```

    ```rust
    impl ASTNode{
        fn is_none(&self) -> bool{
            use self::ASTNode::*;
            match self{
                None=>false,
                _=>true,
            }
        }
    }
    ```
    ```rust
        #[derive(Debug)]
        enum  Expr{
            A,
            B,
            C,
            D,
        }
        fn main() {
            let expr = Expr::B as i32;
            println!("{:?}",expr);

        }
    ```