use std::collections::HashMap;

struct MultipleDimentionalIdArray {
    height: usize,
    width: usize,
    ids: HashMap<i32, (usize, usize)>,
    grid: Vec<i32>,
}

impl MultipleDimentionalIdArray {
    fn new(height: usize, width: usize) -> MultipleDimentionalIdArray {
        let grid = vec![0; height * width];
        let ids: HashMap<i32, (usize, usize)> = HashMap::new();

        MultipleDimentionalIdArray {
            height,
            width,
            ids,
            grid,
        }
    }

    fn get_coordinates_from_id(&self, id: i32) -> Option<&(usize, usize)> {
        self.ids.get(&id)
    }

    fn get_id_from_coordinates(&self, x: usize, y: usize) -> i32 {
        self.grid[y * self.width + x]
    }

    fn cell_is_empty(&self, x: usize, y: usize) -> bool {
        self.grid[y * self.width + x] == 0
    }

    fn add_id(&mut self, id: i32, x: usize, y: usize) -> bool {
        if !self.cell_is_empty(x, y) {
            return false;
        }

        self.grid[y * self.width + x] = id;
        self.ids.insert(id, (x, y));

        true
    }

    fn remove_id(&mut self, id: i32) -> bool {
        let option = self.ids.get(&id);

        if option == None {
            return false;
        }

        let (x, y) = option.unwrap();
        self.grid[y * self.width + x] = 0;

        self.ids.remove(&id);

        true
    }

    fn move_id(&mut self, id: i32, x: usize, y: usize) -> bool {
        if !self.cell_is_empty(x, y) {
            return false;
        }

        if !self.remove_id(id) {
            return false;
        }

        self.add_id(id, x, y)
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<i32> {
        let mut neighbors = Vec::new();

        if x > 0 {
            neighbors.push(self.get_id_from_coordinates(x - 1, y));
        }

        if x < self.width - 1 {
            neighbors.push(self.get_id_from_coordinates(x + 1, y));
        }

        if y > 0 {
            neighbors.push(self.get_id_from_coordinates(x, y - 1));
        }

        if y < self.height - 1 {
            neighbors.push(self.get_id_from_coordinates(x, y + 1));
        }

        neighbors
    }

    // debug function
    fn represent_grid(&self) -> String {
        let mut grid_string = String::new();

        for i in 0..self.height {
            for j in 0..self.width {
                grid_string = format!("{}{}", grid_string, self.grid[i * self.width + j]);
            }
            grid_string = format!("{}\n", grid_string);
        }

        return grid_string;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_id() {
        let mut array = MultipleDimentionalIdArray::new(3, 3);

        let add_id = array.add_id(1, 0, 0);
        assert!(add_id);
        let arr_id = array.add_id(2, 0, 0);
        assert!(!arr_id);
    }

    #[test]
    fn test_get_coordinates_from_id() {
        let mut array = MultipleDimentionalIdArray::new(3, 3);

        array.add_id(1, 0, 0);
        array.add_id(2, 1, 1);
        array.add_id(3, 2, 2);

        assert_eq!(array.get_coordinates_from_id(1), Some(&(0, 0)));
        assert_eq!(array.get_coordinates_from_id(2), Some(&(1, 1)));
        assert_eq!(array.get_coordinates_from_id(3), Some(&(2, 2)));
    }

    #[test]
    fn test_get_id_from_coordinates() {
        let mut array = MultipleDimentionalIdArray::new(3, 3);

        array.add_id(1, 0, 0);
        array.add_id(2, 0, 0);
        array.add_id(3, 1, 1);
        array.add_id(4, 2, 2);

        assert_eq!(array.get_id_from_coordinates(0, 0), 1);
        assert_eq!(array.get_id_from_coordinates(1, 1), 3);
        assert_eq!(array.get_id_from_coordinates(2, 2), 4);
    }

    #[test]
    fn test_cell_is_empty() {
        let mut array = MultipleDimentionalIdArray::new(3, 3);

        array.add_id(1, 0, 0);
        array.remove_id(1);
        array.add_id(3, 1, 1);
        array.add_id(4, 2, 2);

        assert!(array.cell_is_empty(0, 0));
        assert!(array.cell_is_empty(0, 1));
        assert!(!array.cell_is_empty(1, 1));
        assert!(!array.cell_is_empty(2, 2));
    }
}
