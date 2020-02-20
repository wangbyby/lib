# mem::relpace
```rust

pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head,Link::Empty){
            Link::Empty =>  None,
            Link::More(node)=>{
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
```