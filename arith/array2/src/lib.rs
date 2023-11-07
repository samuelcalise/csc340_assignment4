
pub struct Array2<T>{
    pub vec_of_val:Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Array2<T>{

    pub fn new_array(array: Vec<T>, width: usize, height: usize) -> Self{
        Array2{
            vec_of_val: array,
            width,
            height,
        }
    }
                                                        //column, row
    pub fn iter_row_major(&self)-> impl Iterator<Item = (usize, usize, &T)>{
        self.vec_of_val
        .iter()
        .enumerate()
        .map(move |(pos, value)| (pos % self.width, pos / self.width, value))
    }
                                                        //column, row
    pub fn iter_col_major(&self)-> impl Iterator<Item = (usize, usize, &T)>{
        (0..self.width).map(move|element| (element, self.vec_of_val.iter().skip(element)))
        .flat_map(move |(element,col)| {
            col.step_by(self.width)
                .enumerate()
                .map(move |(row,val)| (element,row,val))
        })
    }

    pub fn get_element(& mut self, _col: usize, _row: usize)  -> & mut T{
        &mut self.vec_of_val[(_row * self.width + _col) as usize]
    }

}