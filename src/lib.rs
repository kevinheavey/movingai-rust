use std::ops::Index;

/// Store coorinates in the (x,y) format.
pub type Coords2D = (usize, usize);

pub trait Map2D<T> {
    
    /// Every Map2D must have an height.
    fn get_height(&self) -> usize;

    /// Every Map2D must have a width.
    fn get_width(&self) -> usize;

    /// In every Map2D must be possible to get an item.
    ///
    /// ## Arguments:
    ///  * `coords` (Coords2D) : A tuple representing the desired coordinates.
    ///
    /// ## Examples:
    ///
    /// ```rust
    /// use movingai::Map2D;
    /// use movingai::MovingAiMap;
    ///
    /// let mm = MovingAiMap::new(
    ///        String::from("test"),
    ///        54,
    ///        56,
    ///        vec!['.'; 54*56]
    ///    );
    /// let result = mm.get_cell((23,4));
    /// assert_eq!(*result, '.')
    /// ```
    fn get_cell(&self, coords: Coords2D) -> &T;

}

/// An immutable representation of a MovingAI map.
pub struct MovingAiMap {
    map_type: String,
    height: usize,
    width: usize,
    map: Vec<char>
}

impl MovingAiMap {

    pub fn new(map_type: String, height: usize, width: usize, map: Vec<char>) -> MovingAiMap {
        if map.len() != height*width {
            panic!("Given vector is not compatible with passed `width` and `height`.");
        }
        MovingAiMap {
            map_type, height, width, map
        }
    }

}

impl Map2D<char> for MovingAiMap {

    fn get_width(&self) -> usize { self.width }

    fn get_height(&self) -> usize { self.height }
    
    fn get_cell(&self, coords: Coords2D) -> &char {
        &self.map[coords.1*self.get_width() + coords.0]
    }

}

impl Index<Coords2D> for MovingAiMap  {

    type Output = char;

    fn index(&self, coords: Coords2D) -> &char {
        self.get_cell(coords)
    }
    
}

/// Represent a row (scene) in a scene file.
pub struct SceneRecord {
    bucket: u32,
    map_file: String,
    map_width: usize,
    map_height: usize,
    start_pos: Coords2D,
    goal_pos: Coords2D,
    optimal_length: f64
}

/// Contains all the parser functions.
pub mod parser {

    use std::fs::File;
    use std::io::prelude::*;
    use MovingAiMap;
    use SceneRecord;

    /// Parse a MovingAI `.map` file.
    /// 
    /// # Arguments
    ///  * `path` represents the path to the file location.
    ///
    /// # Returns
    ///  It returns the parsed map as a `MovingAiMap` or an `Err`.
    ///
    /// # Panics
    ///  For the time, it panics if the map format it is not correct.
    ///  TODO: Catch all these errors and encode them into `Result`.
    ///
    /// # Errors
    ///  Return errors if it is not possible to open the specified file.
    ///
    /// # Examples
    ///
    /// ```
    /// use movingai::parser::parse_map_file;
    ///
    /// let map = parse_map_file("./test/arena.map").unwrap();
    /// ```
    pub fn parse_map_file(path: &str) -> Result<MovingAiMap, &'static str> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(err) => {
                panic!("Errore opening file {}", err);
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents);

        let mut height: usize = 0;
        let mut width: usize = 0;
        let mut map_type: String = String::from("empty");
        let mut map: Vec<char> = Vec::new();

        let mut parse_map = false;
        for line in contents.lines() {
            if parse_map {
                for c in line.chars() {
                    map.push(c);
                }
                continue;
            }
            if line.trim() == "map" {
                parse_map = true;
            }
            else {
                let param: Vec<&str> = line.split(" ").collect();
                if param.len() == 2 {
                    let key = param[0];
                    let value = param[1];
                    if key == "type" { map_type = String::from(value); }
                    else if key == "height" { height = value.parse::<usize>().unwrap(); }
                    else if key == "width" { width = value.parse::<usize>().unwrap(); }
                }
            }
        }
        return Ok(MovingAiMap::new(
            map_type, height, width, map
        ));
    }

    /// Parse a MovingAI `.scen` file.
    /// 
    /// # Arguments
    ///  * `path` represents the path to the file location.
    ///
    /// # Returns
    ///  It returns the parsed map as a `Vec<SceneRecord>` or an `Err`.
    ///
    /// # Panics
    ///  For the time, it panics if the map format it is not correct.
    ///  TODO: Catch all these errors and encode them into `Result`.
    ///
    /// # Errors
    ///  Return errors if it is not possible to open the specified file.
    ///
    /// # Examples
    ///
    /// ```
    /// use movingai::parser::parse_scen_file;
    ///
    /// let scen = parse_scen_file("./test/arena2.map.scen").unwrap();
    /// ```
    pub fn parse_scen_file(path: &str) -> Result<Vec<SceneRecord>, &'static str> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(err) => {
                panic!("Errore opening file {}", err);
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents);

        let mut table: Vec<SceneRecord> = Vec::new();

        for line in contents.lines() {
            if line.starts_with("version") {
                continue;
            }
            if (line.is_empty()) {
                continue;
            }
            let record: Vec<&str> = line.split("\t").collect();
            println!("{:?}", record);
            table.push(SceneRecord {
                bucket:  record[0].parse::<u32>().unwrap(),
                map_file: String::from(record[1]),
                map_width: record[2].parse::<usize>().unwrap(),
                map_height: record[3].parse::<usize>().unwrap(),
                start_pos: (record[4].parse::<usize>().unwrap(), record[5].parse::<usize>().unwrap()),
                goal_pos: (record[6].parse::<usize>().unwrap(), record[7].parse::<usize>().unwrap()),
                optimal_length: record[8].parse::<f64>().unwrap()
            })
        }

        return Ok(table);

    }

}

#[cfg(test)]
mod tests {

    use Map2D;
    use MovingAiMap;
    use parser::parse_map_file;
    use parser::parse_scen_file;

    #[test]
    fn indexing() {
        let test = MovingAiMap {
            map_type: String::from("test"),
            height: 4,
            width: 6,
            map: vec!['.'; 4*6]
        };
        assert_eq!(test[(0,3)], '.');
    }

    #[test]
    fn parsing_map() {
        let map = parse_map_file("./test/arena.map").unwrap();
        assert_eq!(map.get_width(), 49 );
    }

    #[test]
    fn parsing_scene() {
        let scen = parse_scen_file("./test/arena2.map.scen").unwrap();
        assert_eq!(scen[3].start_pos,(102, 165));
    }
}
