##不要妄想用dyn trait来模拟类...

##引用, 生命周期
    - 生命周期检查器
    ```rust
        
        fn main() {

            let s1 = String::from("hello");
            let s2 = "world";
            let c = longest(&s1, s2);
        }
        fn longest<'c>(a: &'c str, b: &'c str) -> &'c str{
            if a.len() > b.len() {
                return  a;
            }
            b
        }
    ```
    - mut与&mut
    好奇怪的用法, mut与&mut

    ```rust
        pub fn eval(node:& Box<ast::ASTNode>, mut env:&mut  Environment) ->Box<object::TheObject>
    ```
  - 'static 静态变量

##rust类型系统

  - trait的面向对象特征
    ```rust
    trait Car{
    fn approve(self: Box<Self>) -> Box<dyn Car>;
    }
    #[derive(Debug)]
    struct Tsl{}
    impl Car for Tsl {
        //注意 dyn关键字
        fn approve(self: Box<Self>) -> Box<dyn Car>{
            self
        }
    }
    fn main() {
        let a = Box::new(Tsl{});
        let res = a.approve();
    }
    ```
  - trait的默认行为
    ```rust
    trait Car{
        //默认行为定义
        fn run(&self) -> String {
            String::from("i am running")
        }
    }

    #[derive(Debug)]
    struct Tsl{}

    impl Car for Tsl {
        fn run(&self) -> String {
            String::from("特斯拉在跑")
        }
    }

    struct Wl{}
    //使用默认行为
    impl Car for Wl{}

    fn main() {
        let tsl = Box::new(Tsl{});
        let s = tsl.run();
        println!("{:?}", s);

        let wl = Box::new(Wl{});
        let wres = wl.run();
        println!("{:?}", wres);
    }
    ```
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
 - trait与struct
    - 参数传递
    ```rust
    trait Ani{
    fn hello(&self){}
    }

    struct Cat{}

    impl Ani for Cat{
        fn hello(&self){
            println!("i am a cat!");
        }
    }

    fn say_hello(a:Box<dyn Ani>){
        a.hello();
    }
    fn new() ->Box<dyn Ani>{
        new_cat()
    }

    //加了Option就不行
    fn new_cat()->Box<Cat>{
        Box::new(Cat{})
    }
    fn main() {
        say_hello(new());
        
    }
    ```
    - 类似go的接口断言
    ```rust
    let tests = vec![
        Tests::new("x"),
        Tests::new("y"),
        Tests::new("foobar"),
    ];
    
    
    for (i,v)  in tests.iter().enumerate() {
        let tmp:& Box<dyn ast::Node> = &program.statements[i];
        
        let b: &ast::LetStatement = match tmp.as_any().downcast_ref::<ast::LetStatement>() {//类似接口断言...
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
    /*
    if let Some(ref value) = node.as_any().downcast_ref::<ast::Program>(){
    }else if let Some(ref value)=node.as_any().downcast_ref::<ast::ExpressionStatement>(){
        if let Some(expr) = value.expression{
            return Eval(expr);
        }
    }else if let Some(ref value) = node.as_any().downcast_ref::<ast::IntegerLiteral>(){
    }
    */
    ```
    - class subtyping in rust

    ```rust
    //如果有struct的话, 这个就难办了...
    trait Base : AsBase {
        ...
    }

    trait AsBase {
        fn as_base(&self) -> &Base;
    }

    impl<T: Base> AsBase for T {
        fn as_base(&self) -> &Base { self }
    }
    ```

  - Any的使用
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

##残废的代码

    ```rust
        if let Some(value) = node.as_any().downcast_ref::<ast::Program>() {
            return eval_statements(&value.statements);
        }else if let Some(ref value) = node.as_any().downcast_ref::<ast::ExpressionStatement>(){
            return __eval(&value.expression);
        }else if let Some(ref value) = node.as_any().downcast_ref::<ast::IntegerLiteral>(){
            return Some(Box::new(object::Integer{value: value.value})); 
        }else if let Some(ref value) = node.as_any().downcast_ref::<ast::Boolean>(){
            return Some(nativebool_boolobj(value.value));
        }else if let Some(ref value) = node.as_any().downcast_ref::<ast::PrefixExpression>(){
            return eval_prefix_expr(&value.operator, &__eval(&value.right));
        }else if let Some(ref value) = node.as_any().downcast_ref::<ast::InfixExpression>(){
            return eval_infix_expr(&value.operator, &__eval(&value.left),&__eval(&value.right));
        }else if let Some(ref value) = node.as_any().downcast_ref::<ast::BlockStatement>(){
            return eval_statements(&value.statements);
        }else if let Some(ref value) = node.as_any().downcast_ref::<ast::IfExpression>(){
            return eval_ifexpression(value);
        }
    ```

