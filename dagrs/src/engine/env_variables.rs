use crate::task::DMap;
use anymap::CloneAny;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// Global environment variables
pub struct EnvVar(Arc<Mutex<HashMap<String, DMap>>>);

impl EnvVar {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    #[allow(unused)]
    pub fn set<H: Send + Sync + CloneAny>(&mut self, name: &str, var: H) {
        let mut v = DMap::new();
        v.insert(var);
        self.0.lock().unwrap().insert(name.to_owned(), v);
    }

    #[allow(unused)]
    /// This method get needed input value from [`Inputval`].
    pub fn get<H: Send + Sync + CloneAny>(&self, name: &str) -> Option<H> {
        if let Some(dmap) = self.0.lock().unwrap().get(name) {
            dmap.clone().remove()
        } else {
            None
        }
    }
}

impl Clone for EnvVar {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}