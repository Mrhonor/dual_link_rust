use std::{rc::Rc, cell::RefCell, sync::{Mutex, Arc}, thread, borrow::Borrow, time::Duration, io::{self, Write}};

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
        if pos > self.len{
            panic!("Dual Link error!");
        }

        let mut node = match self.head{
            Option::Some(ref n) => n.clone(),
            Option::None => panic!("Dual Link error!"),
        };
        for _i in 0..pos{
            node = match node.clone().lock().unwrap().next{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
        }

        return node.lock().unwrap().data;
    }

    fn insert(&mut self, data: T, pos: i32){
        if pos > self.len{
            panic!("Dual Link error!");
        }

        if self.len == 0{
            let new_node = Arc::new(Mutex::new(Node { data: data, next: Option::None, prev: Option::None }));
            self.head = Some(new_node.clone());
            let mut lock_node = new_node.lock().unwrap();
            lock_node.next = Some(new_node.clone());
            lock_node.prev = Some(new_node.clone());
       
            self.len += 1;
            return;
        }
        if pos == 0{
            let new_node = Arc::new(Mutex::new(Node { data: data, next: Option::None, prev: Option::None }));
            let mut node = match self.head{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };

            self.head = Some(new_node.clone());
            let mut lock_node = node.lock().unwrap();
            let mut lock_new_node = new_node.lock().unwrap();
            if self.len == 1{
                lock_node.next = Some(new_node.clone());
                lock_node.prev = Some(new_node.clone());
                lock_new_node.next = Some(node.clone());
                lock_new_node.prev = Some(node.clone());
                self.len += 1;
                return;
            }
            else{
                let mut prev_node = match lock_node.prev{
                    Option::Some(ref n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
                let mut lock_prev_node = prev_node.lock().unwrap();
                lock_new_node.next = Some(node.clone());
                lock_new_node.prev = Some(prev_node.clone());
    
                lock_prev_node.next = Some(new_node.clone());
                lock_node.prev = Some(new_node.clone());
           
                self.len += 1;
                return;
            }
        }
        else{
            if self.len == 1{
                let new_node = Arc::new(Mutex::new(Node { data: data, next: Option::None, prev: Option::None }));
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

            let new_node = Arc::new(Mutex::new(Node { data: data, next: Option::None, prev: Option::None }));
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

    fn search(&self, data: T) -> Option<Arc<Mutex<Node<T>>>> {
        if self.len != 0 {
            let mut node = match self.head{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            for _i in 0..self.len{
                if node.lock().unwrap().data == data{
                    return Some(node.clone());
                }
                else{
                    node = match node.clone().lock().unwrap().next{
                        Option::Some(ref n) => n.clone(),
                        Option::None => panic!("Dual Link error!"),
                    };
                }
            }
        }
        return Option::None;
    }

    fn delete(&mut self, pos: i32) -> Option<Arc<Mutex<Node<T>>>>{
        if pos > self.len{
            panic!("Dual Link error!");
        }

        if pos == 0{
            if self.len == 1{
                let node = match self.head{
                    Option::Some(ref n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
                self.head = Option::None;
                self.len -= 1;
                let mut lock_node = node.lock().unwrap();
                lock_node.next = Option::None;
                lock_node.prev = Option::None;
                return Some(node.clone());
            }
            if self.len == 2{
                let node = match self.head{
                    Option::Some(ref n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
                let mut lock_node = node.lock().unwrap();
                
                let next_node = match lock_node.next{
                    Option::Some(ref n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
                self.head = Some(next_node.clone());
                lock_node.next = Option::None;
                lock_node.prev = Option::None;
                let mut lock_next_node = next_node.lock().unwrap();
                lock_next_node.next = Some(next_node.clone());
                lock_next_node.prev = Some(next_node.clone());
                self.len -= 1;
                return Some(node.clone());
            }
            else{
                let node = match self.head{
                    Option::Some(ref n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
                let mut lock_node = node.lock().unwrap();
                let next_node = match lock_node.next{
                    Option::Some(ref n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
                let mut lock_next_node = next_node.lock().unwrap();
                lock_next_node.prev = match lock_node.prev {
                    Option::Some(ref n) => Some(n.clone()),
                    Option::None => panic!("Dual Link error!"),                
                };

                self.head = Some(next_node.clone());
                match lock_node.prev{
                    Option::Some(ref n) => {
                        let mut lock_n = n.lock().unwrap();
                        lock_n.next = Some(next_node.clone())
                    },
                    Option::None => panic!("Dual Link error!"),
                }
                lock_node.next = Option::None;
                lock_node.prev = Option::None;
        
                self.len -= 1;
                return Some(node.clone());
            }
        }
        else{
            let mut prev_node = match self.head{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            for _i in 1..pos{
                prev_node = match &prev_node.clone().lock().unwrap().next{
                    Option::Some(n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
            }

            let mut lock_prev_node = prev_node.lock().unwrap();
            let node = match lock_prev_node.next{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            
            let mut lock_node = node.lock().unwrap();

            let next_node = match lock_node.next{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };

            let mut lock_next_node = next_node.lock().unwrap();

            lock_next_node.prev = Some(prev_node.clone());

            lock_prev_node.next = Some(next_node.clone());
            lock_node.next = Option::None;
            lock_node.prev = Option::None;
       
            self.len -= 1;
            return Some(node.clone());


        }


    }

}

fn main() {

    let mut head = DualLink::new();
    head.insert(2, 0);    
    head.insert(2, 0);    
    head.insert(4, 1);    
    head.insert(5, 2);    

    for i in 0..head.len{
        println!("{} node data: {}", i, head.get(i));
    }

    let search_node = head.search(3);
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.lock().unwrap().data),
        Option::None => println!("search node data: None"),
    }
    
    head.delete(1);
    println!("second node data: {}", head.get(1));
    let search_node = head.search(4);
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.lock().unwrap().data),
        Option::None => println!("search node data: None"),
    }

    head.delete(0);
    println!("first node data: {}", head.get(0));
    let search_node = head.search(2);
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.lock().unwrap().data),
        Option::None => println!("search node data: None"),
    }

    head.delete(0);
    
    let search_node = head.search(5);
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.lock().unwrap().data),
        Option::None => println!("search node data: None"),
    }

    let mut head = DualLink::new();
    head.insert("a", 0);    
    head.insert("b", 1);    
    head.insert("c", 2);    

    for i in 0..head.len{
        println!("{} node data: {}", i, head.get(i));
    }

    let search_node = head.search("b");
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.lock().unwrap().data),
        Option::None => println!("search node data: None"),
    }
    
    head.delete(1);
    println!("second node data: {}", head.get(1));
    let search_node = head.search("b");
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.lock().unwrap().data),
        Option::None => println!("search node data: None"),
    }

    head.delete(0);
    println!("first node data: {}", head.get(0));
    let search_node = head.search("a");
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.lock().unwrap().data),
        Option::None => println!("search node data: None"),
    }
    
    head.delete(0);
    let search_node = head.search("c");
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.lock().unwrap().data),
        Option::None => println!("search node data: None"),
    } 

    let mut head = Arc::new(Mutex::new(DualLink::new()));
    let mut threads = vec![];
    for i in 0..1000{
        let this_head = head.clone();
        threads.push(thread::spawn(move || {
            let mut lock_head = this_head.lock().unwrap();
            lock_head.insert(1, 0);
            drop(lock_head);
            thread::sleep(Duration::from_millis(1));
            let mut lock_head = this_head.lock().unwrap();
            lock_head.insert(2, 1);
            drop(lock_head);
            thread::sleep(Duration::from_millis(1));
            let mut lock_head = this_head.lock().unwrap();
            lock_head.delete(0);
        }));
    }

    for thread in threads{
        thread.join().unwrap();
    }
    let mut lock_head = head.lock().unwrap();
    match lock_head.search(1){
        Option::Some(n) => println!("search for 1: {}", n.lock().unwrap().data),
        Option::None => println!("search for 1: None"),
    }

    match lock_head.search(2){
        Option::Some(n) => println!("search for 2: {}", n.lock().unwrap().data),
        Option::None => println!("search for 1: None"),
    }

    println!("list len: {}", lock_head.len);


}
