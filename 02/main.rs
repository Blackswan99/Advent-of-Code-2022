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
    let mut _score:i32 = 0;
    let mut _ip_string:String;

    
    if let Ok(lines) = read_lines("/home/registereduser/Dev/AoC/Advent-of-Code-2022/dec-02/src/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {

                // A for Rock, B for Paper, and C for Scissors (opponent)
                // X for Rock, Y for Paper, and Z for Scissors (you)
                // Scoring: 1 for Rock, 2 for Paper, and 3 for Scissors
                // Scoring: 6 for winning, 3 for draw, 0 for lose

                _ip_string = String::from(ip);
                     if _ip_string == "A X" { /*println!("draw");*/ _score += 4; }
                else if _ip_string == "A Y" { /*println!("win");*/ _score += 8; }
                else if _ip_string == "A Z" { /*println!("lose");*/ _score += 3; }
                else if _ip_string == "B X" { /*println!("lose");*/ _score += 1; }
                else if _ip_string == "B Y" { /*println!("draw");*/ _score += 5; }
                else if _ip_string == "B Z" { /*println!("win");*/ _score += 9; }
                else if _ip_string == "C X" { /*println!("win");*/ _score += 7; }
                else if _ip_string == "C Y" { /*println!("lose");*/ _score += 2; }
                else if _ip_string == "C Z" { /*println!("draw");*/ _score += 6; }
            }
        }
    } else {
        println!("Nothing to read from file");
    }

    println!("The score for the first round is: {}", _score);
    
    // SECOND PART

    _score = 0;
    let mut vec;
    if let Ok(lines) = read_lines("/home/registereduser/Dev/AoC/Advent-of-Code-2022/dec-02/src/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {

                // A for Rock, B for Paper, and C for Scissors (opponent)
                // X for Rock, Y for Paper, and Z for Scissors (you)
                // Scoring: 1 for Rock, 2 for Paper, and 3 for Scissors
                // Scoring: 6 for winning, 3 for draw, 0 for lose
                /* X means you need to lose,
                   Y means you need to end the round in a draw, and
                   Z means you need to win*/
                _ip_string = String::from(ip);
                vec = _ip_string.split(' ').collect::<Vec<&str>>();

                println!("{:?}",vec[0]);
                /*
                if _ip_string.get(0..0) == Some("A") /*Rock*/ {
                    if _ip_string.get(2..2) == Some("X") {/* go for scissors,*/ _score += 3;}
                    if _ip_string.get(2..2) == Some("Y") {/* go for rock,*/ _score += 4;}
                    if _ip_string.get(2..2) == Some("Z") {/* go for paper,*/ _score += 8;}
                }
                else if _ip_string.get(0..0) == Some("B") /*Paper*/ {
                    if _ip_string.get(2..2) == Some("X") {/* go for rock,*/ _score += 1;}
                    if _ip_string.get(2..2) == Some("Y") {/* go for paper,*/ _score += 5;}
                    if _ip_string.get(2..2) == Some("Z") {/* go for scissors,*/ _score += 9;}
                }
                else if _ip_string.get(0..0) == Some("C") /*Scissors*/ {
                    if _ip_string.get(2..2) == Some("X") {/* go for paper,*/ _score += 2;}
                    if _ip_string.get(2..2) == Some("Y") {/* go for scissors,*/ _score += 6;}
                    if _ip_string.get(2..2) == Some("Z") {/* go for rock, */ _score += 7;}
                }
                */
            }
        }
    } else {
        println!("Nothing to read from file");
    }

    println!("The score for the second round is: {}", _score);
}
