use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
enum Stmt {
    Block(Vec<Stmt>),
    // Let(String, i32),
    // Assign(String, i32),
    Print(String),
}

#[derive(Clone, Default, Debug)]
struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, i32>,
}

impl Environment {
    pub fn wrap(enclosing: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        let environment = Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        };
        Rc::new(RefCell::new(environment))
    }
}

struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        let environment = Rc::new(RefCell::new(Environment::default()));
        Self { environment }
    }
}

impl Interpreter {
    fn new() -> Self {
        Self::default()
    }

    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Block(stmts) => self.execute_block(stmts, Rc::clone(&self.environment)),
            // Stmt::Let(name, val) => self.execute_let(name, val),
            // Stmt::Assign(name, val) => self.execute_assign(name, val),
            Stmt::Print(name) => self.execute_print(name),
        }
    }

    fn execute_block(&mut self, stmts: Vec<Stmt>, _environment: Rc<RefCell<Environment>>) {
        for stmt in stmts {
            self.execute(stmt);
        }
    }

    // fn execute_block(&mut self, stmts: Vec<Stmt>, environment: Rc<RefCell<Environment>>) {
    //     let previous = self.environment.clone();
    //     self.environment = environment;

    //     let res = self.execute_block_inner();

    //     self.environment = previous;

    //     res
    // }

    // fn execute_block_inner(&mut self) {}

    // fn execute_let(&mut self, name: String, val: i32) {}

    // fn execute_assign(&mut self, name: String, val: i32) {}

    fn execute_print(&mut self, name: String) {
        println!(
            "{}: {:?}",
            name,
            self.environment.borrow().values.get(&name)
        );
    }
}

fn main() {
    let mut interpreter = Interpreter::new();
    let stmt = Stmt::Block(vec![
        // Stmt::Let("a".to_string(), 1),
        // Stmt::Assign("a".to_string(), 1),
        Stmt::Print("a".to_string()),
        Stmt::Print("a".to_string()),
    ]);

    interpreter
        .environment
        .borrow_mut()
        .values
        .insert("a".to_string(), 1);
    interpreter.execute(stmt);
}
