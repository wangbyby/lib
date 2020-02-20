
# mod test
```rust
#[cfg(test)] //重点
mod test {
    use super::List; //重点
    #[test]
    fn basic(){   
        let mut list = List::new();
        assert_eq!(list.peek(),None);
        assert_eq!(list.peek_mut(),None);

        assert_eq!(list.pop(),None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(),Some(&3));
        assert_eq!(list.peek_mut(),Some(&mut 3));


        assert_eq!(list.pop(),Some(3));
        assert_eq!(list.pop(),Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(),Some(5));
        assert_eq!(list.pop(),Some(4));
        
        list.peek_mut().map(|value|{
            *value = 100;
        });
        assert_eq!(list.peek(),Some(& 100));
    }
}
```