fn main() {

}

struct LRUCache {
    max_size: i32,
    cache: Vec<(i32, i32)>
}

impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache {
            max_size: capacity,
            cache: Vec::new()
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        for i in 0..self.cache.len() {
            if self.cache[i].0 == key {
                let pair = self.cache.remove(i);
                self.cache.insert(0, pair);
                return pair.1;
            }
        }
        -1
    }
    
    fn put(&mut self, key: i32, value: i32) {
        for i in 0..self.cache.len() {
            if self.cache[i].0 == key {
                self.cache.remove(i);
                break;
            }
        }
        
        self.cache.insert(0, (key, value));
        if self.cache.len() > self.max_size as usize {
            self.cache.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let mut lru = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(1, lru.get(1));
        lru.put(3, 3);
        assert_eq!(-1, lru.get(2));
        lru.put(4, 4);
        assert_eq!(-1, lru.get(1));
        assert_eq!(3, lru.get(3));
        assert_eq!(4, lru.get(4));
    }

    #[test]
    fn example2() {
        let mut lru = LRUCache::new(2);
        assert_eq!(-1, lru.get(2));
        lru.put(2, 6);
        assert_eq!(-1, lru.get(1));
        lru.put(1, 5);
        lru.put(1, 2);
        assert_eq!(2, lru.get(1));
        assert_eq!(6, lru.get(2))
    }
}