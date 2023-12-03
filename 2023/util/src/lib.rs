use std::cmp::min;

pub trait Grid {
    type Item;

    fn get_cell(&self, x: i32, y: i32) -> Option<&Self::Item>;

    fn for_each_2d<F>(&self, f: F)
    where
        F: FnMut(usize, usize, &Self::Item);

    fn get_surrounding(&self, x: usize, y: usize) -> Vec<&Self::Item>;
}

impl<T> Grid for Vec<Vec<T>> {
    type Item = T;

    fn get_cell(&self, x: i32, y: i32) -> Option<&Self::Item> {
        if x < 0 || y < 0 {
            None
        } else {
            self.get(y as usize).and_then(|row| row.get(x as usize))
        }
    }

    fn for_each_2d<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize, &Self::Item),
    {
        for (y, row) in self.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                f(x, y, value);
            }
        }
    }

    fn get_surrounding(&self, x: usize, y: usize) -> Vec<&Self::Item> {
        let mut items = Vec::with_capacity(8);
        for m in y.saturating_sub(1)..min(self.len(), y + 2) {
            for k in x.saturating_sub(1)..min(self[0].len(), x + 2) {
                if k != x || m != y {
                    items.push(&self[m][k]);
                }
            }
        }
        items
    }
}
