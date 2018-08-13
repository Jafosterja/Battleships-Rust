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
    let _player = setup_player();
    draw_board(&_player);
}
pub fn setup_player() -> Player{
    let carrier = create_carrier();
    let bs = create_battleship();
    let cruiser = create_cruiser();
    let sub = create_submarine();
    let dest = create_destroyer();
    
    let player = Player{myships : [carrier,bs,cruiser,sub,dest], alive : true, a_ships : 5};
    return player;
}
pub fn draw_board(player : &Player){
    //Header
    println!("  A B C D E F G H I J", );

    let mut print_string = "".to_string();
    let mut check_ships = false;
    for y in 1..11{//rows
        print_string += &y.to_string();//initial char
        for k in 1..11{//columns for that row
            let cords = Cords{x : convert_int_to_char(k), y : y, hit : false};
            for ships in 0..5{
                if player.myships[ships].check(cords) == true{
                    print_string.push(' ');
                    print_string.push(player.myships[ships].get_type_char());
                    check_ships = true;
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