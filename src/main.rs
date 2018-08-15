mod ship; 
mod tests;
mod player;
use player::*;
use ship::*;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let mut game_mode = String::new(); 
    loop{
        println!("Do you want to host(H) or Connect(C): ");
        io::stdin().read_line(&mut game_mode).expect("failed to read from stdin");//Example found on stackoverflow
        let mut trimmed_gamemode = game_mode.trim().to_uppercase();
        let choice = trimmed_gamemode.pop().unwrap();
        match choice{
            'H' => {play_host();
            break;}
            'C' => {play_connect(); 
            break;}
            _ => continue,
        }
    }
    
}
pub fn play_connect(){
    let mut player = setup_player();
    let mut ip = String::new();
    println!("Enter a valid IP address to connect to: ");
    io::stdin().read_line(&mut ip).expect("failed to read from stdin");//Example found on stackoverflow
    let mut t_ip = ip.trim().to_owned() + ":80";
    println!("{}", t_ip);
    let mut stream = TcpStream::connect(t_ip).expect("Couldn't connect to the server...");
    loop{
        loop{
            let mut inputx = String::new();
            println!("Please enter the column to hit the enemy(A-J): ");
            io::stdin().read_line(&mut inputx).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmedx = inputx.trim().to_uppercase();
            let x_tr = trimmedx.pop().unwrap();
            if check_if_x_is_valid(&x_tr){
                let _ = stream.write(&[convert_char_to_int(&x_tr) as u8]);
                break;
            }   
        }
        loop{
            let mut inputy = String::new();
            println!("Please enter the row to hit the enemy(1-10): ");
            io::stdin().read_line(&mut inputy).expect("failed to read from stdin");//Example found on stackoverflow
            let trimmedy = inputy.trim();
            let x = trimmedy.len();
            let mut y_s = trimmedy.chars().next().unwrap().to_digit(10).unwrap();
            if y_s == 1 && x == 2{
                println!("You entered a value greater than or equal to 10 assuming 10");
                y_s = 10;
            }
                if check_if_y_is_valid(&(y_s as usize)){
                    let _ = stream.write(&[y_s as u8]);
                    break;
                }
            }
            let status = stream.read(&mut [0; 128]);
            let status_c = convert_int_to_char(status.unwrap());
            if status_c == 'W'{
                println!("You Won!");
                break;
            }
            let x_i = stream.read(&mut [0; 128]);
            let x = convert_int_to_char(x_i.unwrap());
            let y = stream.read(&mut [0; 128]);
            let y_t = y.unwrap();
            println!("{}, {}",x,y_t.clone());
            player = player.check_cords(x,y_t);
            let checkstat = player.check_status();
            if checkstat == false{
                let _ = stream.write(&[11]);
                println!("You Lost.");
                break;
            }
            else{
                let _ = stream.write(&[0]);
            }
    }
}
pub fn setup_player() -> Player{
    let mut player : Player = Default::default();
    loop{
         let carrier = create_carrier();
         let test = player.check_if_ship_fits(&carrier);
         if test == true{
             player = player.add_ship(carrier);
             break;
         }
         else{
             println!("Ship creation failed coordinates overlap with already existing ship");
         }
    }
    draw_board(&player);
    loop{
         let bs = create_battleship();
         let test = player.check_if_ship_fits(&bs);
         if test == true{
             player = player.add_ship(bs);
             break;
         }
         else{
             println!("Ship creation failed coordinates overlap with already existing ship");
         }
    }
    draw_board(&player);
    loop{
         let cruiser = create_cruiser();
         let test = player.check_if_ship_fits(&cruiser);
         if test == true{
             player = player.add_ship(cruiser);
             break;
         }
         else{
             println!("Ship creation failed coordinates overlap with already existing ship");
         }
    }
    draw_board(&player);
    loop{
         let sub = create_submarine();
         let test = player.check_if_ship_fits(&sub);
         if test == true{
             player = player.add_ship(sub);
             break;
         }
         else{
             println!("Ship creation failed coordinates overlap with already existing ship");
         }
    }
    draw_board(&player);
    loop{
         let dest = create_destroyer();
         let test = player.check_if_ship_fits(&dest);
         if test == true{
             player = player.add_ship(dest);
             break;
         }
         else{
             println!("Ship creation failed coordinates overlap with already existing ship");
         }
    }
    return player;
}
pub fn draw_board(player : &Player){
    //Header
    println!("   A B C D E F G H I J", );

    let mut print_string = "".to_string();
    let mut check_ships = false;
    for y in 1..11{//rows
        if(y == 10)
        {
            print_string += &y.to_string();
        }
        else
        {
            print_string += &y.to_string();//initial char
            print_string.push(' ');
        }
        for k in 1..11{//columns for that row
            let cords = Cords{x : convert_int_to_char(k), y : y, hit : false};
            let mut check_if_hit = 0;
            for ships in 0..5{

                if player.myships[ships].check(cords) >= 0{
                    check_if_hit = player.myships[ships].check(cords);
                    if player.myships[ships].cordinates[check_if_hit as usize].hit == true{
                        print_string.push(' ');
                        print_string.push('X');
                        check_ships = true;
                    }
                    else{
                        print_string.push(' ');
                        print_string.push(player.myships[ships].get_type_char());
                        check_ships = true;
                    }
                }
            }
            if check_ships == false{
                print_string.push(' ');
                print_string.push('O');
            }
            check_ships = false;
        }
        println!("{}", print_string );
        print_string = "".to_string();
    }
}
pub fn play_host(){
    let mut player = setup_player();
    draw_board(&player);
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let (mut result_sock, result_addr) = listener.accept().unwrap();
    loop{
        let x_i = result_sock.read(&mut [0; 128]);
        let x = convert_int_to_char(x_i.unwrap());
        let y = result_sock.read(&mut [0; 128]);
        let y_t = y.unwrap();
        println!("{}, {}",x,y_t.clone());
        player = player.check_cords(x,y_t);
        let checkstat = player.check_status();
        if checkstat == false{
            let _ = result_sock.write(&[11]);
            println!("You Lost.");
            break;
        }
        else{
            let _ = result_sock.write(&[0]);
            draw_board(&player);
        }
        loop{
            let mut inputx = String::new();
            println!("Please enter the column to hit the enemy(A-J): ");
            io::stdin().read_line(&mut inputx).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmedx = inputx.trim().to_uppercase();
            let x_tr = trimmedx.pop().unwrap();
            if check_if_x_is_valid(&x_tr){
                let _ = result_sock.write(&[convert_char_to_int(&x_tr) as u8]);
                break;
            }   
        }
        loop{
            let mut inputy = String::new();
            println!("Please enter the row to hit the enemy(1-10): ");
            io::stdin().read_line(&mut inputy).expect("failed to read from stdin");//Example found on stackoverflow
            let trimmedy = inputy.trim();
            let x = trimmedy.len();
            let mut y_s = trimmedy.chars().next().unwrap().to_digit(10).unwrap();
            if y_s == 1 && x == 2{
                println!("You entered a value greater than or equal to 10 assuming 10");
                y_s = 10;
            }
                if check_if_y_is_valid(&(y_s as usize)){
                    let _ = result_sock.write(&[y_s as u8]);
                    break;
                }
            }
            let status = result_sock.read(&mut [0; 128]);
            let status_c = convert_int_to_char(status.unwrap());
            if status_c == 'W'{
                println!("You Won!");
                break;
            }
    }
}