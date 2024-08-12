

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




fn main(){
    
}