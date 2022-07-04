use std::fs::File;
use std::io;
use std::io::*;

fn main() -> io::Result<()> {
    let file = File::open("Test//map1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut grid = vec![];
    for line in reader.lines(){
        let mut row = Vec::new();
        for c in line.unwrap().chars(){
            if c != ',' {
                row.push(c);
            }
        }

        grid.push(row);
    }


    Ok(())
}
