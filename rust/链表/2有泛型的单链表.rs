

pub struct List<T>{
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>{
    elem:T,
    next:Link<T>,
}

impl<T> List<T>{
    pub fn new() -> Self{
        List{head:None,}
    }

    pub fn push(&mut self,elem: T){
        let new_node = Box::new(Node{
            elem: elem,
            next: self.head.take(), //cannot move out of borrowed content
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T>{
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }
    pub fn peek_mut(&mut self)-> Option<&mut T>{
        self.head.as_mut().map(|node|{
            &mut node.elem
        })
    }

    pub fn into_iter(self)->IntoIter<T>{
        IntoIter(self)
    }

    pub fn iter(&self)->Iter<'_, T>{
    //与下面等价
    // pub fn iter(&self)->Iter<T>
    // pub fn iter<'a>(&'a self)->Iter<'a, T>
        Iter{
            next: self.head.as_ref().map(|node| &**node)
        }
    }

    pub fn iter_mut(&mut self)-> IterMut<'_,T>{
        IterMut{
            next: self.head.as_mut().map(|node| &mut **node)
        }
    }
}

impl<T> Drop for List<T>{
    fn drop(&mut self){
        let mut cur_link = self.head.take();
        while let Some(ref mut node) = cur_link{
            cur_link = node.next.take();
        }
    }
}


pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>{
        self.0.pop()
    }
}


pub struct Iter<'a,T>{
    next:Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node|{
            self.next = node.next.as_ref().map::<&Node<T>,_>(|node| &node);//或者 map(|node|&**node)
            &node.elem
        })
    }
}



pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T>{
    type Item = &'a mut T;

    //1. We take the Option<&mut> so we have exclusive access to the mutable reference. No need to worry about someone looking at it again.
    //2. Rust understands that it's ok to shard a mutable reference into the subfields of the pointed-to struct, because there's no way to "go back up", and they're definitely disjoint.

    fn next(&mut self) -> Option<Self::Item>{
        self.next.take().map(|node|{
            self.next = node.next.as_mut().map(|node|&mut **node);
            &mut node.elem
        })
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
        
    }

    #[test]
    fn peek(){
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter(){
        let mut list = List::new();
        list.push(1);list.push(2);list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        
    }

    #[test]
    fn iter(){
        let mut list = List::new();
        list.push(1);list.push(2);list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut(){
        let mut list = List::new();
        list.push(1);list.push(2);list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}