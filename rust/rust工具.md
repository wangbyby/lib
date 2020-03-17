## rust工具
  - rustfmt
    ```bash
        #安装rustfmt
         rustup component add rustfmt
        #使用fmt
        cargo fmt
    ```
  - rustfix修复代码
    ```bash
        cargo fix
    ```
  - clippy: 更强大的lint工具 
    ```bash
        #安装clippy
        rustup component add clippy
        #使用clippy
        cargo clippy
    ```

## 属性
  - cfg
    ```rust
    #[cfg(target_os = "linux")]
    fn current_os(){
        println!("you are on linux");
    }
    #[cfg(not(target_os = "linux"))]
    fn current_os(){
        println!("you are **not** on linux");
    }
    fn main() {
        current_os();
    }
    ```

  - 测试
      - cargo test -- --nocapture
      - 单元测试
        ```
        #[cfg(test)] //只在cargo test中生效, 不编译在cargo build中
        ```
        
        
## 第三方包
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
