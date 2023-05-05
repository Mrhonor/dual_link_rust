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
        if pos > self.len{
            panic!("Dual Link error!");
        }

        let mut node = match self.head{
            Option::Some(ref n) => n.clone(),
            Option::None => panic!("Dual Link error!"),
        };
        for _i in 0..pos{
            node = match node.clone().borrow().next{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
        }

        return node.borrow().data;
    }

    fn insert(&mut self, data: T, pos: i32){
        if pos > self.len{
            panic!("Dual Link error!");
        }

        if pos == 0{
            let new_node = Rc::new(RefCell::new(Node { data: data, next: Option::None, prev: Option::None }));
            self.head = Some(new_node.clone());
            new_node.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(new_node.clone());
       
            self.len += 1;
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

            let new_node = Rc::new(RefCell::new(Node { data: data, next: Option::None, prev: Option::None }));
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

    fn search(&self, data: T) -> Option<Rc<RefCell<Node<T>>>> {
        if self.len != 0 {
            let mut node = match self.head{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            for _i in 0..self.len{
                if node.borrow().data == data{
                    return Some(node.clone());
                }
                else{
                    node = match node.clone().borrow().next{
                        Option::Some(ref n) => n.clone(),
                        Option::None => panic!("Dual Link error!"),
                    };
                }
            }
        }
        return Option::None;
    }

    fn delete(&mut self, pos: i32) -> Option<Rc<RefCell<Node<T>>>>{
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
                node.borrow_mut().next = Option::None;
                node.borrow_mut().prev = Option::None;
                return Some(node);
            }

            let node = match self.head{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            let next_node = match node.borrow().next{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            
            next_node.borrow_mut().prev = match node.borrow().prev {
                Option::Some(ref n) => Some(n.clone()),
                Option::None => panic!("Dual Link error!"),                
            };

            self.head = Some(next_node.clone());
            match node.borrow().prev{
                Option::Some(ref n) => n.borrow_mut().next = Some(next_node.clone()),
                Option::None => panic!("Dual Link error!"),
            }
            node.clone().borrow_mut().next = Option::None;
            node.clone().borrow_mut().prev = Option::None;
       
            self.len -= 1;
            return Some(node);
        }
        else{
            let mut node = match self.head{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            for _i in 0..pos{
                node = match &node.clone().borrow().next{
                    Option::Some(n) => n.clone(),
                    Option::None => panic!("Dual Link error!"),
                };
            }

            let next_node = match node.borrow().next{
                Option::Some(ref n) => n.clone(),
                Option::None => panic!("Dual Link error!"),
            };
            
            next_node.borrow_mut().prev = match node.borrow().prev {
                Option::Some(ref n) => Some(n.clone()),
                Option::None => panic!("Dual Link error!"),                
            };

            match node.borrow().prev{
                Option::Some(ref n) => {
                    (*n.clone()).borrow_mut().next = Some(next_node.clone());
                },
                Option::None => panic!("Dual Link error!"),
            }
            node.clone().borrow_mut().next = Option::None;
            node.clone().borrow_mut().prev = Option::None;
       
            self.len -= 1;
            return Some(node);


        }


    }

}

fn main() {

    let mut head = DualLink::new();
    head.insert(2, 0);    
    head.insert(4, 1);    
    head.insert(5, 2);    

    for i in 0..head.len{
        println!("{} node data: {}", i, head.get(i));
    }

    let search_node = head.search(4);
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.borrow().data),
        Option::None => println!("search node data: None"),
    }
    
    head.delete(1);
    println!("second node data: {}", head.get(1));
    let search_node = head.search(4);
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.borrow().data),
        Option::None => println!("search node data: None"),
    }

    head.delete(0);
    println!("first node data: {}", head.get(0));
    let search_node = head.search(2);
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.borrow().data),
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
        Option::Some(n) => println!("search node data: {}", n.borrow().data),
        Option::None => println!("search node data: None"),
    }
    
    head.delete(1);
    println!("second node data: {}", head.get(1));
    let search_node = head.search("b");
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.borrow().data),
        Option::None => println!("search node data: None"),
    }

    head.delete(0);
    println!("first node data: {}", head.get(0));
    let search_node = head.search("a");
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.borrow().data),
        Option::None => println!("search node data: None"),
    }
    
    head.delete(0);
    let search_node = head.search("c");
    match search_node {
        Option::Some(n) => println!("search node data: {}", n.borrow().data),
        Option::None => println!("search node data: None"),
    } 


}
