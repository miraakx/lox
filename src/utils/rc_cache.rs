use std::rc::Rc;

use rustc_hash::FxHashMap;

#[derive(Debug, Default)]
pub struct RcStringCache
{
    data: FxHashMap<String, Rc<String>>
}

impl RcStringCache {
    pub fn get(&mut self, string: String) -> Rc<String> {
        self.data
            .entry(string.clone())
            .or_insert(Rc::new(string))
            .clone()
    }
}
