mod my {
    use std::collections::HashMap;
    use std::rc::{Rc, Weak};  // Rc -> Reference counter
    use std::cell::RefCell;

    #[derive(Debug)]
    pub struct Tree {
        parent: Option<Weak<RefCell<Tree>>>,
        // children: HashMap<String, Rc<RefCell<Tree>>>,
        pub children: RefCell<HashMap<String, Rc<RefCell<Tree>>>>,
    }
    impl Tree {
        pub fn new() -> Self {
            let parent = None;
            let children = RefCell::new(HashMap::new());
            Self { parent, children }
        }
        pub fn with_parent(self, parent: &Rc<RefCell<Tree>>) -> Tree {
            let parent = Some(Rc::downgrade(parent));
            Self { parent, ..self }
        }
        // pub fn add_child(&mut self, name: &str, child: Rc<RefCell<Tree>>) {
        //     let name = name.to_string();
        //     self.children.insert(name, child);
        // }
        // pub fn with_child(mut self, name: &str, child: Rc<RefCell<Tree>>) -> Self {
        //     let name = name.to_string();
        //     self.children.insert(name, child);
        //     Self { ..self }
        // }
        pub fn insert(&mut self, name: &str, child: Rc<RefCell<Tree>>) {
            let name = name.to_string();
            self.children.borrow_mut().insert(name, child);
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use my::Tree;
use std::borrow::BorrowMut;

pub fn solve(input_path: &str) -> String {
    let input: String = std::fs::read_to_string(input_path)
        .expect("failed to read input");

    let mut root = Rc::new(RefCell::new(Tree::new()));
    let d1 = Rc::new(RefCell::new(Tree::new().with_parent(&root)));
    //*root.borrow_mut().children.insert("d1", Rc::clone(&d1));
    //let r = *root.borrow_mut();
    //dbg!(r);
    // *root.borrow_mut() = *root.with_child("d1", d1);
    dbg!(&root.borrow());
    dbg!(&root.borrow_mut());
    *root.borrow_mut().children.insert("d1", d1);
    //*root.borrow_mut().insert("d1", Rc::clone(&d1));

    //"test".to_string()
    "test".to_owned()
}
