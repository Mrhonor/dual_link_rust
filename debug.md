# Usability of Rust
## BUG 1: 在Match中的move
* **错误案例**：Figure 1  

![Match bug](dual_link/img/Option_match.png)  

* **bug分析**：
在match中将borrow()的对象move到了对象n，在代码块结束时n会被销毁，而borrow()对象不能被修改  
* 当时根据编译器的修改结果：如Figure 2所示  

![Match bug fix](dual_link/img/Option_match2.png)  

* **debug分析**：加入&后，被move的只是一个引用，可以被销毁
* 在后续学习过程中，对match和指针的了解加深后，发现更好的修改方式：
```rust
match head.borrow().next{
    Option::Some(ref n) => ...
}
```

## BUG 2: 缺少Clone
* **错误案例**：Figure 3   

![Rc clone](dual_link/img/RC_clone.jpg)  

* **bug分析**：没有对node进行clone()，所有权被node.borrow()借走，无法对node再次赋值  
* **修改结果**：Figure 4  

![Bug fixed](dual_link/img/rc_clone_finished.jpg)

## BUG 3: 无头节点实现双向循环链表时double borrow_mut
* **错误案例**：
```RUST
struct Node<T>{
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}
impl<T> Node<T>{
    fn insert(&mut self, data: T, pos: i32){
        ...
        next_node.clone().borrow_mut().prev = Some(new_node.clone());
        ...
    }
}

fn main(){
    let mut head = Rc::new(RefCell::new(Node::new(0)));
    ...
    head.borrow_mut().insert(data, rear_pos);
}
``` 

* bug分析：在运行时，在双向循环链表插入尾节点过程中，其next_node为头节点，此时再次borrow_mut()将会出现double borrow_mut的出错
* debug方法: 增加判断或修改为带头节点的双向循环链表。本处使用的是带头节点的双向循环链表.
* debug后的代码:

``` RUST
use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
struct Node<T>{
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct DualLink<T>{
    len: i32,
    head: Option<Rc<RefCell<Node<T>>>>,
   
}  

impl<T: PartialEq + Copy> DualLink<T> {    
    fn new() -> DualLink<T>{
        return DualLink { len: 0, head: Option::None}
    }

    fn get(&self, pos: i32) -> T{
        ...
    }

    fn insert(&mut self, data: T, pos: i32){
        ...
        if self.len == 0{
            ...
            return;
        }
        if pos == 0{
            ...
            return;
        }
        else{
            let mut node = match self.head{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            for _i in 1..pos{
                node = match node.clone().borrow().next{
                    Option::Some(ref n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
            }

            let new_node = Rc::new(RefCell::new(
                Node { data: data, next: Option::None, prev: Option::None }));
            let next_node = match node.clone().borrow().next{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            new_node.borrow_mut().next = Some(next_node.clone());
            next_node.clone().borrow_mut().prev = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(node.clone());
            node.borrow_mut().next = Some(new_node.clone());
            
            self.len += 1;
            return;
        }

    }
}
```

## BUG 4: 实现多线程安全双向循环链表时的死锁问题  
* **错误案例**:
```RUST
use std::{rc::Rc, cell::RefCell, sync::{Mutex, Arc}, 
    thread, borrow::Borrow, time::Duration, io::{self, Write}};

#[derive(Debug)]
struct Node<T>{
    data: T,
    next: Option<Arc<Mutex<Node<T>>>>,
    prev: Option<Arc<Mutex<Node<T>>>>,
}

#[derive(Debug)]
struct DualLink<T>{
    len: i32,
    head: Option<Arc<Mutex<Node<T>>>>,
   
}  


impl<T: PartialEq + Copy> DualLink<T> {    
    fn new() -> DualLink<T>{
        return DualLink { len: 0, head: Option::None}
    }

    fn get(&self, pos: i32) -> T{
        ...
    }

    fn insert(&mut self, data: T, pos: i32){
        ...
        if self.len == 0{
            ...
            return;
        }
        if pos == 0{
            ...
        }
        else{
            let mut node = match self.head{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            for _i in 1..pos{
                node = match node.clone().lock().unwrap().next{
                    Option::Some(ref n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
            }

            let new_node = Arc::new(Mutex::new(Node { 
                data: data, next: Option::None, prev: Option::None }));
            let mut lock_node = node.lock().unwrap();
            let next_node = match lock_node.next{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            let mut lock_new_node = new_node.lock().unwrap();
            let mut lock_next_node = next_node.lock().unwrap();
            lock_new_node.next = Some(next_node.clone());
            lock_next_node.prev = Some(new_node.clone());
            lock_new_node.prev = Some(node.clone());
            lock_node.next = Some(new_node.clone());
            
            self.len += 1;
            return;
        }

    }
}
```

* bug分析：在插入时，需要获取插入位置前后两把锁，如果循环链表节点长度为一，将会尝试获取同一把锁两次，导致运行时死锁  
* debug方法：单独处理此类情况 

```RUST
impl<T: PartialEq + Copy> DualLink<T> {    
    fn insert(&mut self, data: T, pos: i32){
        ...
        if self.len == 0{
            ...
            return;
        }
        if pos == 0{
            ...
        }
        else{
            if self.len == 1{
                let new_node = Arc::new(Mutex::new(
                    Node { data: data, next: Option::None, prev: Option::None }));
                let mut node = match self.head{
                    Option::Some(ref n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
                let mut lock_node = node.lock().unwrap();
                lock_node.next = Some(new_node.clone());
                lock_node.prev = Some(new_node.clone());
                let mut lock_new_node = new_node.lock().unwrap();
                lock_new_node.next = Some(node.clone());
                lock_new_node.prev = Some(node.clone());
                self.len += 1;
                return;
            }
            ...
            return;
        }
    }
}
```

## Is the compiler error report useful? If not, can you suggest a better way to report the error? 
在大多数情况下编译器报错较为有用，同时还为每种报错类型提供了一个官方错误案例。但是理解编译器的报错提示需要对RUST所有权和生命周期机制，以及各个API的使用方法有充分的了解。对于初学者更好的方式是需要去阅读官方文档。编译器推荐的debug的方式在部分情况下有效，但并不是最简洁的写法（如bug 1）


