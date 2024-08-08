use std::{cell::RefCell, collections::hash_map::Entry, rc::Rc};
use rustc_hash::FxHashMap;
use string_interner::{StringInterner, Symbol};
use crate::{alias::IdentifierSymbol, values::Value};

#[derive(Clone, Debug)]
pub struct Environment
{
    scope: FxHashMap<IdentifierSymbol, Value>,
    opt_enclosing: Option<Rc<RefCell<Self>>>,
}

impl Environment {

    pub fn default() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { scope: FxHashMap::default(), opt_enclosing: None }))
    }

    pub fn new (enclosing: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        //println!("new enclosing environment count = {}", count_enclosing(&enclosing.borrow())+1);
        Rc::new(RefCell::new(Self { scope: FxHashMap::default(), opt_enclosing: Some(Rc::clone(enclosing)) }))
      }

    pub fn get(&self, name: &IdentifierSymbol) -> Option<Value>  {
        let opt_value: Option<&Value> = self.scope.get(name);

        if let Some(value) = opt_value {
            return Some(value.clone());
        }

        match &self.opt_enclosing {
            Some(enclosing) => {
                return enclosing.borrow().get(name).clone();
            },
            None => {
                None
            },
        }
    }

    pub fn assign(&mut self, name: IdentifierSymbol, value: &Value) -> Result<(), ()>
    {
        if let Entry::Occupied(mut e) = self.scope.entry(name) {
            e.insert(value.clone());
            return Ok(());
        }

        match &self.opt_enclosing {
            Some(enclosing) => {
                enclosing.borrow_mut().assign(name, value)
            },
            None => {
                Err(())
            },
        }
    }

    pub fn define_variable(&mut self, name: IdentifierSymbol, value: Value)
    {
        //println!("enclosing count = {}", count_enclosing(self));
        self.scope.insert(name, value);
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Self>> {
        let mut environment: Rc<RefCell<Environment>> = self.opt_enclosing.as_ref().map(Rc::clone).expect("Initial environment must have an enclosing");
        for _ in 1..distance {
            let next = {
                let current_ref = environment.borrow();
                match &current_ref.opt_enclosing {
                    Some(enclosing) => Rc::clone(enclosing),
                    None => panic!("Ancestor does not exist at the given distance: {}", distance),
                }
            };
            environment = next;
        }
        environment
    }

    pub fn get_at(&self, distance: usize, name: &IdentifierSymbol) -> Option<Value>  {
        if distance == 0 {
            return self.scope.get(name).cloned();
        }
        self.ancestor(distance).borrow().get(name).clone()

    }

    pub fn assign_at(&mut self, distance: usize, name: IdentifierSymbol, value: &Value) -> Result<(), ()> {
        if distance == 0 {
            return self.assign(name, value);
        }
        return self.ancestor(distance).borrow_mut().assign(name, value);
    }

}

pub fn _count_enclosing(env: &Environment) -> usize {
    let mut count = 0;
    let mut current = env.opt_enclosing.as_ref().map(Rc::clone);

    while let Some(enclosing) = current {
        count += 1;
        current = enclosing.borrow().opt_enclosing.as_ref().map(Rc::clone);
    }

    count

}

pub fn _print_env(env: &Rc<RefCell<Environment>>, string_interner: &StringInterner) -> usize {
    let mut count = 0;
    let mut current = env.borrow().opt_enclosing.as_ref().map(Rc::clone);
    println!("count={}", count);
    match &current {
        Some(c) => {
            for (id,_) in c.borrow().scope.iter() {
                println!("id={} -> {}", id.to_usize(), string_interner.resolve(*id).unwrap());
            }
        },
        None => {},
    }

    while let Some(enclosing) = current {
        count += 1;
        current = enclosing.borrow().opt_enclosing.as_ref().map(Rc::clone);
        println!("count={}", count);
        match &current {
            Some(c) => {
                for (id,_) in c.borrow().scope.iter() {
                    println!("id={} -> {}", id.to_usize(), string_interner.resolve(*id).unwrap());
                }
            },
            None => {

            },
        }
    }
    count
}