use std::collections::HashMap;
use std::env;

struct Todo {
    map: HashMap<String, bool>,
}
impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        std::fs::write("db.txt", content)
    }

    fn new() -> Result<Todo, std::io::Error> {
        let content = std::fs::read_to_string("db.txt")?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, "\t").collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), v.parse::<bool>().unwrap()))
            .collect();
        Ok(Todo { map })
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    fn remove(&mut self, key: &String) -> Option<bool> {
        self.map.remove(key)
    }
}

fn main() {
    let action = env::args().nth(1).expect("Please specify an action");
    let item = env::args().nth(2).expect("Please specify an item");

    println!("Started with, action: {}, and item: {}", action, item);

    let mut todo = Todo::new().expect("Initialisation of database failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved!"),
            Err(why) => println!("An error occured: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not in the todo list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved!"),
                Err(why) => println!("Error occured: {}", why),
            },
        }
    } else if action == "remove" {
        match todo.remove(&item) {
            None => println!("{}, is not in the todo list", item),
            Some(_) => println!("Removed todo: '{}' from the list", item),
        }
    }
}
