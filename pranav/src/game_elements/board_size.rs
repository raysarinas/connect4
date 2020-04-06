use crate::game_elements::Dim;

#[derive(Clone, Debug, PartialEq)]
pub struct BoardSize {
    rows: Dim,
    cols: Dim,
}

impl BoardSize {
    pub fn new(rows: Dim, cols: Dim) -> Self {
        assert!(rows >= 1, "Number of rows on the board cannot by less than 1");
        assert!(rows >= 1, "Number of cols on the board cannot by less than 1");
        
        Self {
            rows: rows,
            cols: cols
        }
    }

    pub fn rows(&self) -> Dim {
        self.rows.clone()
    }

    pub fn cols(&self) -> Dim {
        self.cols.clone()
    }
}