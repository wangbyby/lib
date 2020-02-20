use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

pub struct List<T>{
    head: Link<T> ,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T>{
    elem:T,
    next:Link<T>,
    prev:Link<T>,
}

impl<T> Node<T>{
    fn new(elem:T) -> Rc<RefCell<Self>>{
        Rc::new(RefCell::new(Node{
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}
impl<T> List<T>{
    pub fn new() -> Self{
        List{ head:None, tail:None}
    }

    pub fn push_front(&mut self,elem:T){
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(old_node)=>{
                old_node.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_node);
                self.head = Some(new_node);
            },
            None=>{
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            },
        }
    }

    pub fn push_back(&mut self,elem: T){
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail)=>{
                old_tail.borrow_mut().next =  Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            },
            None=>{
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            },
        }
    }

    pub fn pop_front(&mut self) -> Option<T>{
        self.head.take().map(|old_head|{
            match old_head.borrow_mut().next.take() {
                Some(new_head) =>{
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                },
                None =>{
                    self.tail.take();
                },
            }

            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
    pub fn pop_back(&mut self) ->Option<T> {
        self.tail.take().map(|old_tail|{
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail)=>{
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                },
                None =>{
                    self.head.take();
                },
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_back(&self) ->Option<Ref<T>>{
        self.tail.as_ref().map(|node|{
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }
    pub fn peek_front(&self) -> Option<Ref<T>>{
        self.head.as_ref().map(|node|{
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back_mut(&mut self) ->Option<RefMut<T>>{
        self.tail.as_ref().map(|node|{
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn peek_front_mut(&mut self) ->Option<RefMut<T>>{
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn into_iter(self) ->IntoIter<T>{
        IntoIter(self)
    }

    // pub fn iter(&self)->Iter<T>{
    //     Iter(self.head.as_ref().map(|head| head.borrow()))
    // }
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self)->Option<Self::Item>{
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T>{
    fn next_back(&mut self)->Option<Self::Item>{
        self.0.pop_back()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

//TODO
// pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

// impl<'a, T> Iterator for Iter<'a, T>{
//     type Item = Ref<'a, T>;
//     fn next(&mut self) -> Option<Self::Item>{
//         self.0.take().map(|node_ref| {
//             let (next, elem) = Ref::map_split(node_ref, |node | {
//                 (&node.next, &node.elem)
//             });
//             self.0 = next.as_ref().map(|head| head.borrow())
//             elem
//         })

//     }
// }

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);


    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_back_mut().is_none());
        assert!(list.peek_front_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(),3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(),&mut 3);
        assert_eq!(&*list.peek_back().unwrap(),&1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(),&mut 1);

        
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(),Some(3));
        assert_eq!(iter.next_back(),Some(1));
        assert_eq!(iter.next(), Some(2));

        assert_eq!(iter.next_back(),None);
        assert_eq!(iter.next(), None);
    }
}