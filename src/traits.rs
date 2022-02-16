pub trait List<T> {
    /// リストの長さを返す
    fn size(&self) -> usize;
    /// リストの`i`番目の要素を返す
    fn get(&self, i: usize) -> Option<T>;
    /// リストの`i`番目の要素を`v`にする
    fn set(&mut self, i: usize, v: T) -> Option<T>;
    /// `v`を`i`番目の要素として追加し、もとの`i`番目以降の要素を後ろにずらす
    fn add(&mut self, i: usize, v: T);
    /// `i`番目の要素を削除し、もとの`i+1`以降の要素を前にずらす
    fn remove(&mut self, i: usize) -> Option<T>;
}

pub trait Deque<T>: List<T> {
    fn add_first(&mut self, v: T) {
        self.add(0, v)
    }
    fn remove_first(&mut self) -> Option<T> {
        self.remove(0)
    }
    fn add_last(&mut self, v: T) {
        self.add(self.size(), v)
    }
    fn remove_last(&mut self) -> Option<T> {
        self.remove(self.size() - 1)
    }
}

pub trait Queue<T>: Deque<T> {
    fn enqueue(&mut self, v: T) {
        self.add_last(v)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.remove_first()
    }
}

pub trait Stack<T>: Deque<T> {
    fn push(&mut self, v: T) {
        self.add_last(v)
    }

    fn pop(&mut self) -> Option<T> {
        self.remove_last()
    }
}
