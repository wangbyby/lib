记录思维方式和自己的一些感想
wby的study rust
现在写的比较少

总结
- while let Some 和 if let Some 很好使
- 首选递归实现某些算法
    1. c-like遍历二叉树
    ```rust
    //递归的
    pub fn insert_recursive(&mut self,k:K,v:V) {
        if self.key < k {
            if let Some(ref mut right) = self.right {
                right.insert_recursive(k,v);
            }else {
                self.right = Some(Box::new(BinNode::new(k, v))); 
            }
        }else {
            if let Some(ref mut left) = self.left {
                left.insert_recursive(k,v);
            }else {
                self.left = Some(Box::new(BinNode::new(k, v))); 
            }
        }
    }
    //非递归(现在写的还是挺烂的)
    pub fn insert_nonrecursive(&mut self,k:K,v:V) {   
        let mut node = self;
        loop {
            match node.key<=k {
                true => {
                    if let Some(ref mut right) = node.right {
                        node = right;
                    }else{
                        node.right = Some(Box::new(BinNode::new(k, v)));
                        break;
                    }
                },
                false =>{
                    if let Some(ref mut l) = node.left {
                        node = l;
                    }else{
                        node.left = Some(Box::new(BinNode::new(k, v)));
                        break;
                    }
                },
            }
            }
        }
    //后序遍历+计算
    pub fn judge(&mut self) -> Option<Num>{
        match self.key {
            Gkey::op(c) => {
                match (self.left.as_mut(), self.right.as_mut()) {
                    (Some(ref mut l), Some(ref mut r)) => {
                        compute(self.key, l.judge(), r.judge())
                    }
                    _=> None,
                }
            },
            Gkey::num(num)=> Some(num),
            _=> None,
        }
    }
    ```
- 函数式编程
    - map的使用
    ```rust
        // let v = (1..20).map(|x| x+10).collect::<Vec<_>>();
        // for i in &v{
        //     println!("{}",i);
        // }
        let mut rag = rand::thread_rng();
        let mut a  = vec![0;20]; //暂时不指定类型
        for _j in 0..20 {
            let tmp:i8 = rag.gen(); //产生随机数
            a[_j] = tmp;
            
        }
        for i in &a{
            println!("{}",i);
        }
        let bb = a.iter().map(|&x| if x<0 {-x}else{x}).collect::<Vec<_>>(); //不是过滤掉...
        for i in &bb{
            println!("{}",i);
        }
    ```
    - filer 
    ```rust
    fn main() {
        let mut x = vec![0; 10];
        let c = x.iter().filter(|&x| *x!=10).collect::<Vec<_>>();
        for i in c.iter(){
            println!("{}",i);
        }
    }
    ```
    - 综合使用
    ```rust
    pub fn child_nodes_hashmap(&mut self, a: usize) -> HashMap<usize, T> {
        self.payload[a].clone().iter().enumerate().filter(|&(x,y)| *y!=self.default_value).map(|(x,y)| (x,*y)).collect::<HashMap<_,_>>()
    }
    ```