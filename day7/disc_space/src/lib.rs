use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum DiscElement {
    Dict(Rc<RefCell<Dict>>),
    File(File),
}

#[derive(Clone)]
pub struct Dict {
    pub name: String,
    pub content: Vec<DiscElement>,
    pub size: Option<u64>,
    pub parent: Option<Rc<RefCell<Dict>>>,
}

impl Debug for Dict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dict")
            .field("name", &self.name)
            .field("content", &self.content)
            .field("size", &self.size)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct File {
    name: String,
    size: u64,
}

impl File {
    pub fn new<S: Into<String>>(name: S, size: u64) -> Self {
        Self {
            name: name.into(),
            size,
        }
    }
}

pub enum ChangeCommand {
    MoveOut,
    MoveIn(String),
    Home,
}

impl Dict {
    pub fn new(name: String) -> Self {
        Self {
            name,
            content: Vec::new(),
            size: None,
            parent: None,
        }
    }

    pub fn with_parent(name: String, parent: Rc<RefCell<Dict>>) -> Self {
        Self {
            name,
            content: Vec::new(),
            size: None,
            parent: Some(parent),
        }
    }

    pub fn calculate_size(&mut self) -> u64 {
        let mut size = 0;
        for element in &self.content {
            size += match element {
                DiscElement::Dict(d) => d.as_ref().borrow_mut().calculate_size(),
                DiscElement::File(f) => f.size,
            }
        }

        self.size = Some(size);
        size
    }

    pub fn cd(self_: Rc<RefCell<Self>>, command: ChangeCommand) -> Option<Rc<RefCell<Dict>>>
    where
        Self: Sized,
    {
        match command {
            ChangeCommand::MoveOut => match &self_.as_ref().borrow().parent {
                Some(parent) => Some(Rc::clone(parent)),
                None => None,
            },
            ChangeCommand::MoveIn(target) => {
                match self_.as_ref().borrow().content.iter().find(|f| match f {
                    DiscElement::Dict(dict) => {
                        if dict.as_ref().borrow().name.eq(&target) {
                            true
                        } else {
                            false
                        }
                    }
                    DiscElement::File(_) => false,
                }) {
                    Some(x) => match x {
                        DiscElement::Dict(dict) => Some(Rc::clone(dict)),
                        DiscElement::File(_) => panic!("Cannot move into a File!"),
                    },
                    None => None,
                }
            }
            ChangeCommand::Home => {
                if self_.as_ref().borrow().parent.is_none() {
                    let x = Rc::clone(&self_);
                    Some(x)
                } else {
                    let x = Rc::clone(self_.as_ref().borrow().parent.as_ref().unwrap());
                    Dict::cd(x, ChangeCommand::Home)
                }
            }
        }
    }
}
