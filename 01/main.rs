use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
  let mut number = 0;
  let mut elfs_count = 0;
  let mut _elfs: [i32;241] = [0;241];
  let mut elfs_iterator = 0;

  // File hosts must exist in current path before this produces output
  if let Ok(lines) = read_lines("input") {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
        if let Ok(ip) = line {
              if ip.len() > 0 {
                  _elfs[elfs_iterator] += ip.parse::<i32>().unwrap();
              }
              else {
                  elfs_count += 1;
                  elfs_iterator += 1;
              }
          }
      }
  }
  let mut first:i32 = 0;
  let mut second:i32 = 0;
  let mut third:i32 = 0;
  let mut i:usize = 0;

    first = _elfs[0];
    while i <_elfs.len() {
 		  if first < _elfs[i] {
 			  first = _elfs[i];
 		  }
 		  i = i + 1;
    }

    second = _elfs[0];
    i = 0;
    while i < _elfs.len()
    {
 		if second < _elfs[i] {
            if _elfs[i] < first {
 			     second = _elfs[i];
            }
 		}
 		i = i + 1;
    }

    third = _elfs[0];
    i = 0;
    while i < _elfs.len() {
      if third < _elfs[i] {
        if _elfs[i] < second {
          third = _elfs[i];
        }
      }
 		  i = i + 1;
    }

    println!("Elf with most calories has: {}", first);
    println!("Elf with second most calories has: {}", second);
    println!("Elf with third most calories has: {}", third);

    println!("The total amount of the first three places is: {}", first+second+third);
}
