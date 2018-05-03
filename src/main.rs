extern crate rand;

use rand::Rng;
use std::process::Command;
use std::io;


// Just for clearing the screen
fn clear() {
    let mut cmd;
    if cfg!(target_os = "windows") {
        cmd = Command::new("cls");
    } else {
        cmd = Command::new("clear");
    }
    cmd.status();
}


fn print_field(field: [[i8; 3]; 3]) {
    clear();
    println!();
    for i in &field {
        for j in i {
            let x: String = if *j == 0 { 
                    " ".to_string() 
                } else { 
                    j.to_string()
                };
            print!(" {}", x);
        }
        println!();
    }
    println!();
}


fn fill_field(field: &mut [[i8; 3]; 3]) {
    let mut rng = rand::thread_rng();
    let numbs = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut i = 0;
    while i < 8 {
        let r_x: usize = (rng.gen::<f32>() * 3.0) as usize;
        let r_y: usize = (rng.gen::<f32>() * 3.0) as usize;
        if field[r_x][r_y] == 0 {
            field[r_x][r_y] = numbs[i as usize];
            i += 1;
        }
    }
}


fn is_finished(field: [[i8; 3]; 3]) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if field[i][j] != (3 * i + j + 1) as i8 || field[i][j] != 0 {
                return false;
            }
        }
    }
    return true;
}


fn get_inpt() -> i8 {
    loop {
        let mut inpt: String = "".to_string();
        println!("Enter a number you want to move.");
        io::stdin().read_line(&mut inpt);
        inpt.truncate(1);
        match inpt.parse::<i8>() {
            Ok(res) => return res,
            Err(_) => println!("Entered value is no number!")
        }
    }
}


fn get_pos(field: [[i8; 3]; 3], v: i8) -> [i8; 2] {
    let mut x = 0;
    let mut y = 0;
    for i in &field {
        for j in i {
            if *j == v {
                return [x, y];
            }
            x += 1;
        }
        y += 1;
    }
    return [1,2];
}


fn main() {

    let mut field = [ [0; 3]; 3 ];

    fill_field(&mut field);

    while !is_finished(field) {
        print_field(field);
        let chosen_pos = get_pos(field, get_inpt());
        let zero_pos = get_pos(field, 0);
    }
    
}