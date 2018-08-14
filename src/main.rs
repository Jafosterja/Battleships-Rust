mod ship; 
mod tests;
mod player;
use player::Player;
use ship::Cords;
use ship::create_battleship;
use ship::create_carrier;
use ship::create_cruiser;
use ship::create_submarine;
use ship::create_destroyer;


fn main() {
    println!("Hello, world!");
    let mut _player = setup_player();
    draw_board(&_player);
    _player = _player.check_cords('A', 10);
    draw_board(&_player);
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
pub fn convert_int_to_char(input : usize) -> char{
    match input{
        1 => return 'A',
        2 => return 'B',
        3 => return 'C',
        4 => return 'D',
        5 => return 'E',
        6 => return 'F',
        7 => return 'G',
        8 => return 'H',
        9 => return 'I',
        10 => return 'J',
        _ => return 'Z'
    }

}