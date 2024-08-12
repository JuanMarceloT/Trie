

trait Identifiable {
    fn id(&self) -> i32;
}

impl Identifiable for i32 {
    fn id(&self) -> i32 {
        *self
    }
}

impl Identifiable for char {
    fn id(&self) -> i32 {
        let r = (*self) as i32;
        r
    }
}
#[derive(Default, Debug)]
struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    modulo: usize,
}

impl<K:Eq + Identifiable, V> HashMap <K, V> {
    fn new(modulo: usize) -> Self {
        let mut buckets = Vec::with_capacity(modulo);
        for _ in 0..modulo {
            buckets.push(Vec::new());
        }
        HashMap { buckets, modulo }
    }

    fn hash(&self, key: &K) -> usize {
        (key.id() as usize) % self.modulo
    }

    fn insert(&mut self, key: K, value: V) {
        let bucket = self.hash(&key);
        self.buckets[bucket].push((key, value));
    }

    fn get(&mut self, key: &K) -> Option<&mut V>
    where
        K: Eq,
    {
        let bucket_index = self.hash(key);
        for &mut (ref existing_key, ref mut existing_value) in &mut self.buckets[bucket_index] {
            if (*existing_key) == (*key) {
                return Some(existing_value);
            }
        }
        None
    }

}



#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(26),
            is_end_of_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            if node.children.get(&ch).is_none() {
                node.children.insert(ch, TrieNode::new());
            }
            node = node.children.get(&ch).unwrap();
        }
        node.is_end_of_word = true;
    }

    fn search(&mut self, word: &str) -> bool {
        let mut node = &mut self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }
        //println!("{:?}", node.children);
        node.is_end_of_word
    }

    fn starts_with(&mut self, prefix: &str) -> bool {
        let mut node = &mut self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }
        true
    }
}

fn main() {
    let mut trie = Trie::new();

    trie.insert("hello");
    trie.insert("helium");

    println!("{}", trie.search("hello")); // true
    println!("{}", trie.search("hell"));   // false
    println!("{}", trie.starts_with("hellp"));   // false
    println!("{}", trie.starts_with("hell"));   // true
}