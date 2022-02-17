pub trait List<T> {
    /// リストの長さを返す
    fn size(&self) -> usize;
    /// リストの`i`番目の要素を返す
    fn get(&self, i: usize) -> Option<&T>;
    /// リストの`i`番目の要素を`v`にする
    fn set(&mut self, i: usize, v: T) -> Option<&T>;
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

/// 重複なし、順序なしの集合。UはunorderedのU。
pub trait USet<T> {
    /// 集合の要素数を返す
    fn size(&self) -> usize;
    /// `v`が集合に入っていなければ追加する。`v`が集合に追加されたら`true`、そうでなければ`false`を返す
    fn add(&mut self, v: T) -> bool;
    /// 集合から`v`と同じ要素を削除する。削除されたらその要素を、同じ要素が見つからなければ`None`を返す
    fn remove(&mut self, v: &T) -> Option<T>;
    /// `v`と同じ要素を探す。見つかったらその要素を、見つからなければ`None`を返す
    fn find(&self, v: &T) -> Option<&T>;
}

/// ソートされた要素の集合。SはsortedのS。
pub trait SSet<T> {
    /// 集合の要素数を返す
    fn size(&self) -> usize;
    /// `v`が集合に入っていなければ追加する。`v`が集合に追加されたら`true`、そうでなければ`false`を返す
    fn add(&mut self, v: T) -> bool;
    /// 集合から`v`と同じ要素を削除する。削除されたらその要素を、同じ要素が見つからなければ`None`を返す
    fn remove(&mut self, v: &T) -> Option<T>;
    /// `w` ≧ `v`を満たす最小の要素`w`を見つける。`w`が存在すれば`Some(w)`を返し、存在しなけれ`None`を返す
    fn find(&self, v: &T) -> Option<&T>;
}
