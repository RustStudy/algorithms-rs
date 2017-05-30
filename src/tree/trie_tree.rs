use std::collections::hash_map::HashMap;
use std::hash::Hash;

struct Trie<K, V> where K: Eq+Hash+Clone, V: Clone {
    value: Option<V>,
    children: HashMap<K, Trie<K, V>>,
}

impl<K, V> Trie<K,V> where K: Eq+Hash+Clone, V: Clone {
    fn new() -> Trie<K, V> {
        Trie {
            value: None,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, path: Vec<K>, v: V) {
        if path.is_empty() {
            match self.value {
                Some(_) => {
                    panic!("key exists")
                },
                None => {
                    self.value = Some(v);
                },
            }
            return;
        }

        self.children.entry(path[0].clone())
            .or_insert(Trie::new())
            .insert(path[1..].to_vec(), v)
    }

    fn fetch(&self, path: Vec<K>) -> Option<V> {
        match path.len() {
            0 => self.value.clone(),
            _ => self.children.get(&path[0])
                    .unwrap()
                    .fetch(path[1..].to_vec())
        }
    }
}
