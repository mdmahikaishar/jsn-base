use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::fs;
use std::io;

#[allow(dead_code)]
pub struct Connection {
    uri: String,
}

impl Connection {
    /// Create new connection
    /// 
    /// `uri` is a path of a folder, where data will be stored.
    pub fn new(uri: &str) -> Result<Self, io::Error> {
        let _ = fs::create_dir_all(&uri)?;

        Ok(Self {
            uri: uri.to_string(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model<T> {
    name: String,
    data: Vec<T>,
    ty: PhantomData<T>,
    uri: String,
}

impl<T: Serialize + for<'a> Deserialize<'a>> Model<T> {
    /// Create a new Modal instance.
    ///
    /// `name` is name of the modal.
    /// `connection` connection reference.
    pub fn new(name: &str, connection: &Connection) -> Self {
        Self {
            name: name.to_string(),
            data: Vec::new(),
            ty: PhantomData::<T>,
            uri: connection.uri.to_string(),
        }
    }

    fn get_path(&self) -> String {
        format!("{}/{}.jsnbase", &self.uri, &self.name)
    }

    /// load
    ///
    /// load previous state of data.
    pub fn load(&mut self) {
        if let Ok(contents) = std::fs::read_to_string(&self.get_path()) {
            if let Ok(prased_contents) = serde_json::from_str::<Model<T>>(&contents) {
                self.data = prased_contents.data;
            };

            // TODO: handle Error
        }
    }

    /// save
    ///
    /// save the current state of data.
    pub fn save(&self) {
        // TODO: handle error

        let contents = serde_json::to_string(&self).unwrap();

        let _ = std::fs::write(&self.get_path(), &contents);
    }

    /// create
    ///
    /// creates a new record
    pub fn create(&mut self, payload: T) -> Option<&T> {
        self.data.push(payload);

        self.data.get(self.data.len() - 1)
    }

    /// creates
    ///
    /// create many new record.
    pub fn creates(&mut self, payloads: Vec<T>) -> () {
        payloads.into_iter().for_each(|item| {
            self.create(item);
        });
    }

    /// get
    ///
    /// get a single item.
    pub fn get(&self, condition: impl Fn(&T) -> bool) -> Option<&T> {
        self.data.iter().find(|item| condition(*item))
    }

    /// gets
    ///
    /// get many item.
    pub fn gets(&self, condition: impl Fn(&T) -> bool) -> Vec<&T> {
        self.data.iter().filter(|item| condition(*item)).collect()
    }

    /// remove
    ///
    /// remove a single item.
    pub fn remove(&mut self, condition: impl Fn(&T) -> bool) -> Option<T> {
        match self.data.iter().position(|item| condition(item)) {
            Some(index) => Some(self.data.remove(index)),
            _ => None,
        }
    }

    /// removes
    ///
    /// remove many item.
    pub fn removes(&mut self, condition: impl Fn(&T) -> bool) {
        while let Some(_) = self.remove(&condition) {
            //
        }
    }
}
