// Rohan Kelley
//! A two dimensional polymorphic array written as,
//! 'Array2<T>'

/// A struct containing a nested vector of generic type T,
/// a width, and a height
#[derive(Clone)]
pub struct Array2<T: Clone> {
    array_2d: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T: Clone> Array2<T> {
    /// Returns the width of the 2d array, also referred
    /// to as the amount of 'columns'
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the 2d array, also referred
    /// to as the amount of 'rows'
    pub fn height(&self) -> usize {
        self.height
    }

    /// Constructs a new 2d array from a vector, read in from row major form
    ///
    /// # Arguments
    ///
    /// * 'elements': a vector containing elements of generic type
    /// * 'width': a usize containing the amount of columns in the 2d array
    /// * 'height': a usize containing the amount of rows in the 2d array
    pub fn from_row_major(elements: Vec<T>, width: usize, height: usize) -> Self {
        let mut nested_vec: Vec<_> = vec![];
        for i in 0..height {
            // Any Array2 created by row-major form will first be stored in columns 0 - width
            // and then rows 0 - height.
            nested_vec.push(
                elements
                    .iter()
                    .skip(i * width)
                    .map(|val| val.clone())
                    .take(width)
                    .collect(),
            );
        }
        // Any Array2 will have an associated width and height.
        Array2 {
            array_2d: nested_vec,
            width,
            height,
        }
    }

    /// Constructs a new 2d array from a vector, read in from col major form
    ///
    /// # Arguments
    ///
    /// * 'elements': a vector containing elements of generic type
    /// * 'width': a usize containing the amount of columns in the 2d array
    /// * 'height': a usize containing the amount of rows in the 2d array
    pub fn from_col_major(elements: Vec<T>, width: usize, height: usize) -> Self {
        let mut nested_vec: Vec<_> = vec![];
        for i in 0..height {
            // Any Array2 created by column-major form will first be stored in rows 0 - height
            // and then columns 0 - width.
            nested_vec.push(
                elements
                    .iter()
                    .skip(i)
                    .step_by(height)
                    .map(|val| val.clone())
                    .collect(),
            );
        }
        // Any Array2 will have an associated width and height.
        Array2 {
            array_2d: nested_vec,
            width,
            height,
        }
    }

    /// Constructs a new 2d array that has a single value copied at each element
    ///
    /// # Arguments
    ///
    /// * 'singular_value': the value that will be copied to each element in
    ///  the 2d array
    /// * 'width': a usize containing the amount of columns in the 2d array
    /// * 'height': a usize containing the amount of rows in the 2d array
    pub fn from_blank_state(singular_value: T, width: usize, height: usize) -> Self {
        // Any Array2 will have an associated width and height.
        let vec_singular_value = vec![singular_value; width * height];
        // Any Array2 created by row-major form will first be stored in columns 0 - width
        // and then rows 0 - height.
        Array2::from_row_major(vec_singular_value, width, height)
    }

    /// Returns an iterator that will iterate over the 2d array in row major
    /// order. The tuple is set up as (row, column, value)
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        // Any Array2 must be able to be iterated over by row major
        // utilized from Professor Daniel's Array2
        self.array_2d
            .iter()
            .flatten()
            .enumerate()
            .map(move |(index, val)| (index / self.width, index % self.width, val))
    }

    /// Returns an mutable iterator that will iterate over the 2d array in row major
    /// order. The tuple is set up as (row, column, value)
    pub fn iter_row_major_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        // Any Array2 must be able to be iterated over by row major
        // utilized from Professor Daniel's Array2
        self.array_2d
            .iter_mut()
            .flatten()
            .enumerate()
            .map(|(index, val)| (index / self.width, index % self.width, val))
    }

    /// Returns an iterator that will iterate over the 2d array in column major
    /// order. The tuple is set up as (row, column, value)
    pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        let mut col_iter_vec: Vec<_> = vec![];
        for col_idx in 0..self.width {
            let column_vec: Vec<_> = self
                .array_2d
                .iter()
                .enumerate()
                .map(|(row_idx, row_vec)| (row_idx, col_idx, row_vec.get(col_idx).unwrap()))
                .collect();
            col_iter_vec.push(column_vec);
        }
        // Any Array2 must be able to be iterated over by column major
        col_iter_vec
            .into_iter()
            .flat_map(|inner_vec| inner_vec.into_iter())
    }

    /// Returns a reference to an element from the 2d array given the column and
    /// row position
    ///
    /// # Arguments
    ///
    /// * 'row_idx': a usize stating the chosen row index
    /// * 'column_idx': a usize stating the chosen column index
    pub fn get(&self, row_idx: usize, column_idx: usize) -> Option<&T> {
        // Any element in the Array2 should have two indices corresponding to the respective column
        // and row position, which is width and height.
        self.array_2d[row_idx].get(column_idx)
    }

    /// Returns a mutable reference to an element from the 2d array given the column and
    /// row position
    ///
    /// # Arguments
    ///
    /// * 'row_idx': a usize stating the chosen row index
    /// * 'column_idx': a usize stating the chosen column index
    pub fn get_mut(&mut self, row_idx: usize, column_idx: usize) -> Option<&mut T> {
        self.array_2d[row_idx].get_mut(column_idx)
    }

    /// Updates a single cell in the 2d array.
    ///
    /// # Arguments
    ///
    /// * 'row_idx': a usize stating the chosen row index
    /// * 'column_idx': a usize stating the chosen column index
    /// * 'value': the new value to be inserted into the 2d array
    pub fn insert(&mut self, row_idx: usize, column_idx: usize, value: T) {
        self.array_2d[row_idx][column_idx] = value
    }
}
