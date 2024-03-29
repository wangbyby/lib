use std::mem;

// 最普通的单向链表

pub struct List{
    head: Link,
}

enum Link{
    Empty,
    More(Box<Node>),
}

struct Node{
    elem:i32,
    next:Link,
}

impl List{
    pub fn new() -> Self{
        List{head:Link::Empty,}
    }

    pub fn push(&mut self,elem: i32){
        let new_node = Box::new(Node{
            elem: elem,
            next: mem::replace(&mut self.head,Link::Empty), //cannot move out of borrowed content
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head,Link::Empty){
            Link::Empty =>  None,
            Link::More(node)=>{
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

//默认drop是低效的
impl Drop for List{
    fn drop(&mut self){
        let mut cur_link = mem::replace(&mut self.head,Link::Empty);
        while let Link::More(ref mut node) = cur_link{
            cur_link = mem::replace(&mut node.next, Link::Empty);
        }
    }
}




#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basic(){
        
        let mut list = List::new();

        assert_eq!(list.pop(),None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(),Some(3));
        assert_eq!(list.pop(),Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(),Some(5));
        assert_eq!(list.pop(),Some(4));
        
        
        assert_eq!(list.pop(),Some(1));

        assert_eq!(list.pop(),None);
    }
}