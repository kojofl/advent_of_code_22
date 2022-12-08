use std::rc::Rc;

enum DiscElement {
    Dict(Rc<Dict>),
    File(File)
} 


struct Dict {
    name: String,
    content: Vec<DiscElement>,
    size: Option<u64>,
    parent: Option<Rc<Dict>>
}

struct File {
    name: String,
    size: u64
}

enum ChangeCommand {
    MoveOut,
    MoveIn(String),
    Home
}

impl Dict {
    pub fn new(name: String) -> Self {
        Self {
            name,
            content: Vec::new(),
            size: None,
            parent: None
        }
    }

    pub fn with_parent(name: String, parent: Rc<Dict>) -> Self {
        Self {
            name, 
            content: Vec::new(),
            size: None,
            parent: Some(parent)
        }
    }

    pub fn cd(&self, command: ChangeCommand) -> Option<Rc<Dict>> {
        match command {
            ChangeCommand::MoveOut => match &self.parent {
                Some(parent) => Some(Rc::clone(parent)),
                None => None,
            },
            ChangeCommand::MoveIn(target) => {
                match self.content.iter().find(|f| { 
                    match f {
                        DiscElement::Dict(name) => if name.name.eq(&target) {
                            true
                        } else { 
                            false
                        },
                        DiscElement::File(_) => false,
                    }
                }) {
                    Some(x) => todo!(),
                    None => todo!(),
                }
            },
            ChangeCommand::Home => todo!(),
        }
    } 
}

