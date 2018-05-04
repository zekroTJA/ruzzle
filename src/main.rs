extern crate rand;

use rand::Rng;
use std::process::Command;
use std::io;
use std::env;


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


fn print_field(field: [[i8; 3]; 3], moves: i32, d: bool) {
    println!("{}", if d { "DEBUG MODE ENABLED\n" } else { "" });
    println!("Moves:  {}\n", moves);
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


fn fill_field(field: &mut [[i8; 3]; 3], d: bool) {
    if d {
        for i in 0..2 {
            for j in 0..3 {
                field[i][j] = (3 * i + j + 1) as i8;
            }
        }
        field[2][0] = 7;
        field[2][1] = 0;
        field[2][2] = 8;
    } else {
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
}


fn is_finished(field: [[i8; 3]; 3]) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            println!("{} = {}", field[i][j], (3 * i + j + 1));
            if field[i][j] != (3 * i + j + 1) as i8 && field[2][2] != 0 {
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
    let mut y = 0;
    for i in &field {
        let mut x = 0;
        for j in i {
            if *j == v {
                return [x, y];
            }
            x += 1;
        }
        y += 1;
    }
    return [0, 0];
}


fn debug_mode() -> bool {
    let args: Vec<_> = env::args().collect();
    return args.len() > 1 && args[1] == "-d";    
}


fn main() {

    let mut field = [ [0; 3]; 3 ];
    let mut moves: i32 = 0;

    let DEBUG_MODE = debug_mode();

    fill_field(&mut field, DEBUG_MODE);

    while !is_finished(field) {
        clear();
        print_field(field, moves, DEBUG_MODE);
        let chosen_pos = get_pos(field, get_inpt());
        let zero_pos = get_pos(field, 0);
        let diff = ((
            (zero_pos[0] - chosen_pos[0]) * (zero_pos[0] - chosen_pos[0]) +
            (zero_pos[1] - chosen_pos[1]) * (zero_pos[1] - chosen_pos[1])
        ) as f64).sqrt();
        if diff == 1.0 {
            field[zero_pos[1] as usize][zero_pos[0] as usize] = 
                field[chosen_pos[1] as usize][chosen_pos[0] as usize];
            field[chosen_pos[1] as usize][chosen_pos[0] as usize] = 0;
            moves += 1;
        }
    }
    clear();
    print_field(field, moves, DEBUG_MODE);
    println!("Finished in {} move{}!", moves, 
        if moves > 1 { "s" } else { "" });
}
