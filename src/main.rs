use std::env;
use std::fs;

fn debug(_val: &str) {
    #[cfg(debug_assertions)]
    println!("{}", _val);
}

fn pad(val: &str, size: usize) -> String {
    let spaces = size - val.len();
    let padding = " ".repeat(spaces);

    return val.to_owned() + padding.as_str();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &String = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Cannot Open File");

    debug("Got File Contents");
   
    let lines = contents.split("\n");
    let mut cells = vec![vec![""; 100]; lines.clone().count()];

    let mut longest: usize = 0;
    let mut xlen:    usize = 0;
    
    for (line_num, line) in lines.enumerate() {
        if line == "" {
            continue;
        }
        let values = line.split(",");
        xlen = values.clone().count();
        for (index, val) in values.enumerate() {
            cells[line_num][index] = val;
            let length = val.len();
            if length > longest {
                longest = length;
            }
        }
    }

    longest = longest + 1;

    // Parsed data into cells
    // TODO: Print saved data
    
    xlen = (xlen * (longest + 1)) - 1;
    println!("┌{}┐", "─".repeat(xlen));

    for line in cells {
        if line[0] == "" {
            continue;
        }
        for val in line.clone() {
            if val == "" {
                continue; 
            }
            print!("│{}", pad(val, longest));
        }
        //println!("│\x1b[4m");
        println!("│");
        for val in line {
            if val == "" {
                continue;
            }
            //print!("│{}", "─".repeat(longest));
            print!("┼{}", "─".repeat(longest));
        }
        println!("\x1b[24m│");
    }
    println!("└{}┘", "─".repeat(xlen));
}
