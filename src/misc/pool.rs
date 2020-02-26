
trait PoolEq {
    fn eq(&self, id: u32) -> bool;
}

struct Page<T: PoolEq + Default> {
    items: Vec<T>,
    capacity: usize,
    length: usize
}

impl<T: PoolEq + Default> Page<T> {
    pub fn new() -> Page<T> {
        Page {
            items: (0..256).map(|_| T::default()).collect(),
            capacity: 256,
            length: 0
        }
    }

    pub fn is_full(&self) -> bool {
        self.length == self.capacity
    }

    pub fn has_space(&self) -> bool {
        self.length < self.capacity
    }

    pub fn add(&mut self, item: T) -> Result<&T, ()> {
        if self.is_full() {
            return Err(())
        }

        self.items[self.length] = item;
        self.length += 1;

        Ok(&self.items[self.length - 1])
    }

    pub fn free(&mut self, id: u32) -> bool {
        if self.length == 0 {
            return false
        }

        let item_slice = &self.items[0..self.length];
        let item_index = match item_slice.iter().position(|x| x.eq(id)) {
            Some(val) => val,
            None => return false
        };

        self.items.swap(item_index, self.length );
        self.items.remove(self.length);
        self.length -= 1;

        true
    }

    pub fn get(&mut self, id: u32) -> Option<&mut T> {
        if self.length == 0 {
            return None
        }
        self.items.iter_mut().take(2);
        let item_slice = &mut self.items[0..self.length];
        item_slice.iter_mut().find(|x| x.eq(id))
    }
}
/*
struct Pool<T: PoolEq + Default> {
    items: Vec<Box<Page<T>>>
}

impl<T: PoolEq + Default> Pool<T> {
    pub fn new() -> Pool<T> {
        Pool {
            items: Vec::new()
        }
    }

    pub fn add(&mut self, item: T) -> Option<&T> {
        let n = Box::new(Page::new());
        self.items.push(n);

        format!("{}", self.items.len());
        None

    }

    pub fn get(&self, id: u32) -> Option<&T> {
        for i in &self.items {
            let p = i.get(id);

            if p.is_some() {
                return p;
            }
        }
        None
    }
}*/