use std::{collections::HashSet, fmt::Debug};
use itertools::{enumerate, zip_eq, Itertools};

#[derive(Debug, PartialEq)]
pub enum Vec2DError {
    UnBalancedRowsErr,
    InsertRowErr,
    InsertColErr,
    SetValueErr,
    FromStrErr,
}

pub struct Vec2D<T>
    where T : Clone {
    pub width: usize,
    pub height: usize,
    _length: usize,
    pub data: Vec<T>
}

impl<T> Vec2D<T> 
    where T: Clone {
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        return if row < self.height && col < self.width {
            Some(&self.data[row * self.width + col])
        }
        else {
            None
        };
    }

    pub fn get_val(&self, row: usize, col: usize) -> Option<T> {
        return if row < self.height && col < self.width {
            Some(self.data[row * self.width + col].clone())
        }
        else {
            None
        };
    }

    pub fn set(&mut self, val: T, row: usize, col: usize) -> Result<(), Vec2DError> {
        if row >= self.height || col >= self.width  {
            return Err(Vec2DError::SetValueErr);
        }

        self.data[row * self.width + col] = val;

        Ok(())
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            _length: width * height,
            data: Vec::<T>::new()
        }
    }

    pub fn from(source: Vec<Vec<T>>) -> Result<Self, Vec2DError> {
        let height = source.len();
        if height == 0 {
            return Ok(Self {
                height,
                width: 0,
                _length: 0,
                data: Vec::new()
            });
        }

        let width: usize;
        if source.iter().map(|row| row.len()).collect::<HashSet<_>>().len() == 1 {
            width = source[0].len();
        }
        else {
            return Err(Vec2DError::UnBalancedRowsErr);
        }

        let mut data = Vec::<T>::new();
        for row in source {
            for item in row {
                data.push(item);
            }
        }

        Ok(Self {
            width,
            height,
            _length: width * height,
            data
        })
    }

    pub fn insert_row(&mut self, row: Vec<T>, i: usize) -> Result<(), Vec2DError> {
        if i > self.height {
            return Err(Vec2DError::InsertRowErr);
        }

        for item in row.iter().rev() {
            self.data.insert(i * self.width, item.clone());
        }

        self.height += 1;

        Ok(())
    }

    pub fn insert_col(&mut self, col: Vec<T>, i: usize) -> Result<(), Vec2DError> {
        if i > self.width {
            return Err(Vec2DError::InsertColErr);
        }

        let mut i = i;
        for item in col.iter() {
            self.data.insert(i, item.clone());
            i += self.width+1;
        }

        self.width += 1;

        Ok(())
    }

    pub fn repeat_row(&mut self, item: T, i: usize) {
        for _ in 0..(self.width) {
            self.data.insert(i*self.width, item.clone());
        }
        self.height += 1;
    }

    pub fn repeat_col(&mut self, item: T, i: usize) {
        let mut i = i;
        for _ in 0..(self.height) {
            self.data.insert(i, item.clone());
            i += self.width+1;
        }
        self.width += 1;
    }
    
    pub fn repeat_replace_row(&mut self, item: T, i: usize) -> Result<(), Vec2DError> {
        for col in 0..self.height {
            self.set(item.clone(), i, col)?
        }

        Ok(())
    }

    pub fn replace_row(&mut self, items: Vec<T>, row: usize) -> Result<(), Vec2DError> {
        for (item, i) in zip_eq(items, 0..self.height) {
            self.set(item, row, i)?
        }

        Ok(())
    }

    pub fn repeat_replace_col(&mut self, item: T, col: usize) -> Result<(), Vec2DError> {
        for row in 0..self.width {
            self.set(item.clone(), col, row)?
        }

        Ok(())
    }

    pub fn replace_col(&mut self, items: Vec<T>, col: usize) -> Result<(), Vec2DError> {
        for (item, i) in zip_eq(items, 0..self.width) {
            self.set(item, i, col)?
        }

        Ok(())
    }

    pub fn rows(&self) -> Rows<T> {
        let mut items = Vec::<Vec<T>>::new();

        for row_id in 0..self.height {
            let mut row = Vec::<T>::new();
            for col_id in 0..self.width {
                row.push(self.get_val(row_id, col_id).unwrap());
            }
            items.push(row);
        }

        Rows {
            items,
            index: 0
        }
    }

    pub fn row(&self, i: usize) -> Vec<T> {
        let mut items = Vec::<T>::new();

        for item in (i*self.width)..((i*self.width) + self.width) {
            items.push(self.data[item].clone());
        }

        items
    }

    pub fn cols(&self) -> Cols<T> {
        let mut items = Vec::<Vec<T>>::new();

        for col_id in 0..self.width {
            let mut col = Vec::<T>::new();
            for row_id in 0..self.height {
                col.push(self.get_val(row_id, col_id).unwrap());
            }
            items.push(col);
        }

        Cols { items, index: 0 }
    }

    pub fn col(&self, i: usize) -> Vec<T> {
        let mut items = Vec::<T>::new();

        let mut row = 0; 
        while row < self.height {
            items.push(self.get(row, i).unwrap().clone());
            row += 1;
        }

        items
    }

    pub fn split_row(&self, i: usize) -> (Rows<T>, Rows<T>) {
        let mut left: Vec<Vec<T>> = Vec::new();
        let mut right: Vec<Vec<T>> = Vec::new();

        for (index, row) in enumerate(self.rows()) {
            if index < i {
                left.push(row);
            }
            else {
                right.push(row);
            }
        }

        (
            Rows { items: left, index: 0 },
            Rows { items: right, index: 0 }
        )
    }

    pub fn split_col(&self, i: usize) -> (Cols<T>, Cols<T>) {
        let mut top: Vec<Vec<T>> = Vec:: new();
        let mut bot: Vec<Vec<T>> = Vec:: new();

        for (index, col) in enumerate(self.cols()) {
            if index < i {
                top.push(col);
            }
            else {
                bot.push(col);
            }
        }

        (
            Cols { items: top, index: 0 },
            Cols { items: bot, index: 0 }
        )
    }
}

impl Vec2D<std::primitive::char> {
    pub fn from_str(content: &str) -> Result<Vec2D<char>, Vec2DError>{
        if content.split('\n').map(|elem| elem.len()).filter(|elem| *elem != 0).unique().count() != 1 {
            return Err(Vec2DError::FromStrErr);
        }

        let mut data = Vec::<char>::new();
        let width = content.split('\n').next().unwrap().len();
        let mut height = 0;

        for line in content.split('\n') {
            if line.is_empty() { break }
            for char in line.chars() {
                data.push(char);
            }
            height += 1;
        }

        let _length = data.len();

        Ok(Vec2D {
            data,
            _length,
            width,
            height
        })
    }
}

pub struct Cols<T>
    where T : Clone {
    items: Vec<Vec<T>>,
    index: usize
}

impl<T> Iterator for Cols<T>
    where T : Clone {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.items.len() {
            return None;
        }

        let res = &self.items[self.index];
        self.index += 1;

        Some(res.to_vec())
    }
}

pub struct Rows<T>
    where T : Clone {
    items: Vec<Vec<T>>,
    index: usize
}

impl<T> Iterator for Rows<T>
    where T : Clone {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.items.len() {
            return None;
        }

        let res = &self.items[self.index];
        self.index += 1;

        Some(res.to_vec())
    }
}


impl<T> IntoIterator for Vec2D<T> 
    where T : Clone {
    type Item = T;
    type IntoIter = Vec2DIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
       Vec2DIntoIter {
            item: self,
            row: 0,
            col: 0
        }
    }
}

pub struct Vec2DIntoIter<T>
    where T : Clone {
    item: Vec2D<T>,
    row: usize,
    col: usize
}

impl<T> Iterator for Vec2DIntoIter<T> 
    where T : Clone {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.item.get_val(self.row, self.col);

        if self.col >= self.item.width-1 {
            self.col = 0;
            self.row += 1;
        }
        else {
            self.col += 1;
        }

        result
    }
}

#[cfg(test)]
mod vec2d_tests {
    use itertools::assert_equal;
    use crate::data_structures::Vec2DError;
    use super::Vec2D;
    
    pub fn data() -> Vec<Vec<u32>> {
        vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]
    }
    pub fn vec2d() -> Vec2D<u32> {
        Vec2D { width: 3, height: 3, _length: 9, data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9] }
    }

    #[cfg(test)]
    mod create {
        use super::Vec2D;

        #[test]
        fn create() {
            let vec = Vec2D::<u32>::from(super::data()).unwrap();

            assert_eq!(vec.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            assert_eq!(vec.width, 3);
            assert_eq!(vec.height, 3);
        }

        #[test]
        fn from_str() {
            let vec = Vec2D::<char>::from_str("abc\ndef").unwrap();

            assert_eq!(vec.data, vec!['a','b','c','d','e','f']);
        }
    }

    #[cfg(test)]
    mod get {
        #[test]
        fn get_element() {
            let vec = super::vec2d();

            assert_eq!(vec.get(0, 0), Some(&1));
            assert_eq!(vec.get(1, 1), Some(&5));
            assert_eq!(vec.get(2, 0), Some(&7));
        }

        #[test]
        fn get_element_value() {
            let vec = super::vec2d();

            assert_eq!(vec.get_val(0, 0), Some(1));
            assert_eq!(vec.get_val(1, 1), Some(5));
            assert_eq!(vec.get_val(2, 0), Some(7));
        }

        #[test]
        fn get_out_of_bounds() {
            let vec = super::vec2d();

            assert_eq!(vec.get(3, 3), None);
            assert_eq!(vec.get(4, 0), None);
            assert_eq!(vec.get(0, 4), None);
        }
    }

    #[cfg(test)]
    mod set {
        #[test]
        fn set_element() {
            let mut vec = super::vec2d();

            let _ = vec.set(10, 0, 0);
            let _ = vec.set(11, 2, 1);

            assert_eq!(vec.data[0], 10);
            assert_eq!(vec.data[7], 11);
        }


        #[test]
        fn set_out_of_bounds() {
            let mut vec = super::vec2d();

            let res = vec.set(10, 3, 3);

            if let Err(error) = res {
                assert_eq!(error, super::Vec2DError::SetValueErr);
            }
            else {
                assert!(false);
            }
        }
    }

    #[cfg(test)]
    mod insert_row {
        #[test]
        fn insert_row_start() {
            let mut vec = super::vec2d();

            let _ = vec.insert_row(vec![9, 9, 9], 0);

            assert_eq!(vec.data, vec![9, 9, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            assert_eq!(vec.height, 4);
        }

        #[test]
        fn insert_row_mid() {
            let mut vec = super::vec2d();

            let _ = vec.insert_row(vec![9, 9, 9], 1);

            assert_eq!(vec.data, vec![1, 2, 3, 9, 9, 9, 4, 5, 6, 7, 8, 9]);
            assert_eq!(vec.height, 4);
        }

        #[test]
        fn insert_row_end() {
            let mut vec = super::vec2d();

            let _ = vec.insert_row(vec![9, 9, 9], vec.height);

            assert_eq!(vec.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9, 9]);
            assert_eq!(vec.height, 4);
        }

        #[test]
        fn insert_row_out_of_bounds() {
            let mut vec = super::vec2d();

            if let Err(error) = vec.insert_row(vec![9, 9, 9], 5) {
                assert_eq!(error, super::Vec2DError::InsertRowErr);
            }
            else {
                assert!(false);
            }
        }

        #[test]
        fn insert_row_too_large() {
            let mut vec = super::vec2d();

            if let Err(error) = vec.insert_row(vec![9, 9, 9, 9], 0) {
                assert_eq!(error, super::Vec2DError::InsertRowErr);
            }
            else {
                assert!(false);
            }
        }

        #[test]
        fn insert_row_too_small() {
            let mut vec = super::vec2d();

            if let Err(error) = vec.insert_row(vec![9, 9], 0) {
                assert_eq!(error, super::Vec2DError::InsertRowErr);
            }
            else {
                assert!(false);
            }
        }

        #[test]
        fn inesrt_row_repeat() {
            let mut vec = super::vec2d();

            vec.repeat_row(0, 1);

            assert_eq!(vec.data, vec![1, 2, 3, 0, 0, 0, 4, 5, 6, 7, 8, 9]);
        }
    }

    #[cfg(test)]
    mod insert_col {
        #[test]
        fn insert_col_start() {
            let mut vec = super::vec2d();

            let _ = vec.insert_col(vec![9, 9, 9], 0);

            assert_eq!(vec.data, vec![9, 1, 2, 3, 9, 4, 5, 6, 9, 7, 8, 9]);
            assert_eq!(vec.width, 4);
        }

        #[test]
        fn insert_col_mid() {
            let mut vec = super::vec2d();

            let _ = vec.insert_col(vec![9, 9, 9], 1);

            assert_eq!(vec.data, vec![1, 9, 2, 3, 4, 9, 5, 6, 7, 9, 8, 9]);
            assert_eq!(vec.width, 4);
        }

        #[test]
        fn insert_col_end() {
            let mut vec = super::vec2d();

            let _ = vec.insert_col(vec![9, 9, 9], vec.width);

            assert_eq!(vec.data, vec![1, 2, 3, 9, 4, 5, 6, 9, 7, 8, 9, 9]);
            assert_eq!(vec.width, 4);
        }

        #[test]
        fn insert_col_out_of_bounds() {
            let mut vec = super::vec2d();

            if let Err(error) = vec.insert_col(vec![9, 9, 9], 5) {
                assert_eq!(error, super::Vec2DError::InsertColErr);
            }
            else {
                assert!(false);
            }
        }
    }

    #[cfg(test)]
    mod iterators {
        #[test]
        fn iterator_values() {
            let vec = super::vec2d();

            super::assert_equal(vec.into_iter(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter())
        }

        #[test]
        fn iterator_rows() {
            let vec = super::vec2d();

            super::assert_equal(vec.rows(), vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]].into_iter())
        }

        #[test]
        fn iterator_cols() {
            let vec = super::vec2d();

            super::assert_equal(vec.cols(), vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]].into_iter());
        }
    }

    #[cfg(test)]
    mod splits {
        #[test]
        fn split_rows_mid() {
            let vec = super::vec2d();

            let (left, right) = vec.split_row(1);

            super::assert_equal(left, vec![vec![1, 2, 3]].into_iter());
            super::assert_equal(right, vec![vec![4, 5, 6], vec![7, 8, 9]].into_iter());
        }

        #[test]
        fn split_cols_mid() {
            let vec = super::vec2d();

            let (left, right) = vec.split_col(1);

            super::assert_equal(left, vec![vec![1, 4, 7]].into_iter());
            super::assert_equal(right, vec![vec![2, 5, 8], vec![3, 6, 9]].into_iter());
        }
    }

    #[cfg(test)]
    mod replacing {
        use super::vec2d;

        #[test]
        fn replace_row_mid() {
            let mut vec = vec2d();

            let _ = vec.replace_row(vec![0, 0, 0], 1);

            assert_eq!(vec.data, vec![1, 2, 3, 0, 0, 0, 7, 8, 9]);
        }
    }
}
