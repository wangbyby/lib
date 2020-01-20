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
