extern crate termios;
use std::io;
use std::io::Read;
use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use rand::Rng;

fn render(game_16x16: &Vec<&str>) {
    let mut ctr = 1;
    let mut out = "".to_owned();
    for byte in game_16x16 {
        out = out + byte;
        if ctr == 32 {
            out = out + "\n";
            ctr = 0;
        }
        ctr = ctr + 1;
    }
    print!("{}", out);
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn input(termios: Termios) -> Vec<u8> {
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, &mut new_termios).unwrap();

    let mut buffer = [0;1];
    io::stdout().lock().flush().unwrap();
    io::stdin().read_exact(&mut buffer).unwrap();
    tcsetattr(0, TCSANOW, & termios).unwrap();
    return buffer.to_vec();
}

fn main() {
    let termios = Termios::from_fd(0).unwrap();
    let mut game_16x16 = vec!["."; 512];
    let mut player: usize = 256;
    let mut enemies: Vec<usize> = Vec::new();
    let mut streak = 0;
    enemies.push(player);
    loop {
        if enemies.contains(&player) {
            streak = streak + 1;
            enemies.pop();
            enemies.push(rand::thread_rng().gen_range(0..511));
        }
        game_16x16 = vec!["."; 512];
        game_16x16[player] = "#";
        for i in enemies.iter() {
            game_16x16[*i] = "-";
        }
        clear();
        println!("Streak: {streak}");
        render(&game_16x16);
        let key = input(termios);
        if key[0] == 119 {
            if player > 31 {
                player = player - 32;
            }
        } else if key[0] == 115 {
            if player < 480 { 
                player = player + 32;
            }
        } else if key[0] == 100 {
            if player < 511 {
                player = player + 1;
            }
        } else if key[0] == 97 {
            if player > 0 {
                player = player - 1;
            }
        }

    }
}
