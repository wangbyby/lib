
>参考
https://doc.rust-lang.org/book/ch15-04-rc.html


# Rc<T> 引用计数智能指针
the Reference Countered Smart Pointer
不可变引用计数，即可以有多个所有权。
举个例子：家里一起看电视，最后一个人离开时关掉电视📺

# RefCell<T> 内部可变性
即在&self的函数内也能改变内部的值。
注意⚠️：
  - RefCell其实是运行期的借用检查，只能有一个可变引用，多个不可变引用
  - 如果违背了运行时借用检查，panic等着你哦


# 组合 Rc与RefCell
举个例子，Rust的链表😄
～内存泄露等着你😈，循环引用🤣～ 
```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

```


# Weak<T> 弱引用出来吧

>
Strong references are how you can share ownership of an Rc<T> instance. 
Weak references don’t express an ownership relationship.
They won’t cause a reference cycle 
because any cycle involving some weak references will be broken 
once the strong reference count of values involved is 0.

- 强弱引用转换！
   - 强到弱：downgrade
   - 若到强：upgrade

举个🌰,   带父节点的二叉树
```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};


///比C/C++复杂好多😄
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

```
