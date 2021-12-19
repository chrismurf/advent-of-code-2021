use std::fs::File;
use std::collections::HashSet;
use std::io::{self, prelude::*, BufReader};


#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point { x: usize, y: usize }

#[derive(Debug, PartialEq, Clone, Copy)]
struct Path {
    previous: Option<Point>,
    risk: Option<u32>
}

impl Path {
    fn empty() -> Self {
        Path { previous: None, risk: None }
    }
}


#[derive(Debug, PartialEq, Clone)]
struct CavernMap {
    map : Vec<Vec<u8>>,
    width: usize,
    height: usize
}

impl CavernMap {
    fn from_file(path : &str) -> Self {
        let mut map : Vec<Vec<u8>> = Vec::new();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            map.push( line.trim().bytes().map(|b| b - '0' as u8).collect() );
        }
        let width = if map.is_empty() { 0 } else { map[0].len() };
        let height = map.len();
        CavernMap { map, width, height }
    }

    fn get_neighbors(&self, p: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        if p.x > 0 { neighbors.push( Point { x: p.x-1, y: p.y } ); }
        if p.x < self.width-1 { neighbors.push( Point { x: p.x+1, y: p.y } ); }
        if p.y < self.height-1 { neighbors.push( Point { x: p.x, y: p.y+1 } ); }
        if p.y > 0 { neighbors.push( Point { x: p.x, y: p.y-1 } ); }
        neighbors
    }

    fn safest_path(&self) -> (Vec<Point>, u32) {
        // Implement Djikstra's algorithm.  For each point, we keep track of 
        // the best previous step to get there, and the cost it took.  We then
        // iterate through every element (in order of current cost) et voila.
        let mut safest_paths : Vec<Vec<Path>> = vec![
            vec![Path::empty(); self.width]; self.height
        ];

        safest_paths[0][0] = Path {previous: None, risk: Some(0) };

        let mut unvisited: HashSet<Point> = HashSet::new();
        for x in 0..self.width {
            for y in 0..self.height {
                unvisited.insert(Point { x, y } );
            }
        }
        
        while !unvisited.is_empty() {
            // Find current lowest risk path.  This should be done with a Priority Queue,
            // but I'm being lazy and just iterating through the whole set each time.
            let mut point: Point = Point { x: 255, y: 255 };
            let mut lowest_cost = u32::MAX;
            for unvis_point in &unvisited {
                let path = &safest_paths[unvis_point.y][unvis_point.x];
                if path.risk.unwrap_or(u32::MAX) < lowest_cost {
                    point = *unvis_point;
                    lowest_cost = path.risk.unwrap_or(u32::MAX);
                }
            }
            unvisited.remove(&point);

            // Now propagate to each neighbor, if our candidate path is best
            let path_to_point = safest_paths[point.y][point.x];

            for nbr in self.get_neighbors(&point) {
                let nbr_risk = self.map[nbr.y][nbr.x];
                let path_to_nbr = &safest_paths[nbr.y][nbr.x];
                let new_nbr_risk = path_to_point.risk.unwrap() + nbr_risk as u32;
                let current_nbr_risk = path_to_nbr.risk.unwrap_or(u32::MAX);
                if new_nbr_risk < current_nbr_risk {
                    safest_paths[nbr.y][nbr.x].previous = Some(point);
                    safest_paths[nbr.y][nbr.x].risk = Some(new_nbr_risk);
                }
            }

        }

        // Now find the best path and cost 
        let end_cell = safest_paths[self.height-1][self.width-1];

        (Vec::new(), end_cell.risk.unwrap())
    }
}

fn main() -> io::Result<()> {
    let mut cavern_map = CavernMap::from_file("input.txt");
    let (safest_path, cost) = cavern_map.safest_path();
    println!("Safest path has cost {}", cost);
    
    Ok(())
}
