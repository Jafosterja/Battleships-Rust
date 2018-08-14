use std::io;

#[derive(Copy,Clone)]
pub struct Cords{
    pub x :  char, 
    pub y :  i8, 
    pub hit : bool,
}
impl Default for Cords {
    fn default() -> Cords {
        Cords {
            x : ' ',
            y : 0,
            hit : true
        }
    }
}
#[derive(Clone)]
pub struct Ship{
    pub shiptype : String,
    //pub coordinates: Vec<Cords>,
    pub status : bool,
    pub index : i8,
    pub cordinates : [Cords; 5],
}
impl Default for Ship {
    fn default() -> Ship {
        Ship {
            shiptype : "Default".to_string(),
            status : true,
            index : 0,
            cordinates : [Default::default(),Default::default(),Default::default(),Default::default(),Default::default()],
        }
    }
}
impl Ship{
    pub fn check_if_hit(mut self, test : Cords) -> Ship{
        for mut p in 0..5{
            if self.cordinates[p].hit == true {
                continue;
            }
            else{
                if self.cordinates[p].y == test.y && self.cordinates[p].x == test.x{
                    self.cordinates[p].hit = true;
                    return self;
                }
            }
        }
        return self;
    }
    pub fn check_if_dead(self) -> bool{
        let mut x = true;
        for mut p in 0..5{
            if self.cordinates[p].hit == false{
                x = false;
                return x;
            }

        }
        return x;
    }
    pub fn add_cords(&mut self, cord : Cords){
        self.cordinates[self.index as usize] = cord;
    }
    pub fn get_type(&self) -> &String{
        return &self.shiptype;
    }
    pub fn check(&self, test : Cords) -> isize{
        for range in 0..5{
             if test.x == self.cordinates[range].x && test.y == self.cordinates[range].y {
                return range as isize;
             }
        }
        return -1
    }
    pub fn get_type_char(&self) -> char{
        match self.shiptype.as_ref(){
            "Carrier" => return 'C',
            "Battleship" => return 'B',
            "Cruiser" => return 'R',
            "sub" => return 'S',
            "Destroyer" => return 'D',
            _ => return 'U'
        }
    }
}
pub fn create_carrier() -> Ship{
    let mut inputx = String::new();
    let mut inputy = String::new();
    let mut input_dir = String::new();
    loop{//Loop till we build a valid ship
        let mut dir_initial = ' ';
        let mut x_initial = ' ';
        let mut y_initial = 0;
        loop{
            println!("Please enter the column for the anchor point of the carrier(A-J): ");
            io::stdin().read_line(&mut inputx).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmedx = inputx.trim().to_uppercase();
            x_initial = trimmedx.pop().unwrap();
            if check_if_x_is_valid(&x_initial){
                break;
            }   
        }
        loop{
            println!("Please enter the row for the anchor point of the carrier(1-10): ");
            io::stdin().read_line(&mut inputy).expect("failed to read from stdin");//Example found on stackoverflow
            let trimmedy = inputy.trim();
            let x = trimmedy.len();
            y_initial = trimmedy.chars().next().unwrap().to_digit(10).unwrap();
            if y_initial == 1 && x == 2{
                println!("You entered a value greater than or equal to 10 assuming 10");
                y_initial = 10;
            }
            if check_if_y_is_valid(&(y_initial as usize)){
                break;
            }
        }
        loop{
            println!("Please enter A direction to face the carrier(N,S,E,W): ");
            io::stdin().read_line(&mut input_dir).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmed_dir = input_dir.trim().to_uppercase();
            dir_initial = trimmed_dir.pop().unwrap();
            if check_if_dir_valid(&dir_initial){
                break;
            }
        }
        if check_setup(&dir_initial, &x_initial, &(y_initial as usize), 4){
            if dir_initial == 'N'{
                let s = convert_char_to_int(&x_initial);
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : x_initial, y : (y_initial as i8) - 1, hit : false};
                let c = Cords{x : x_initial, y : (y_initial as i8) - 2, hit : false};
                let d = Cords{x : x_initial, y : (y_initial as i8) - 3, hit : false};
                let e = Cords{x : x_initial, y : (y_initial as i8) - 4, hit : false};
                let carrier = Ship{ shiptype : "Carrier".to_string(), cordinates: [a,b,c,d,e], status : false, index : 0};
                return carrier;
            }
            if dir_initial == 'S'{
                let s = convert_char_to_int(&x_initial);

                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : x_initial, y : (y_initial as i8) + 1, hit : false};
                let c = Cords{x : x_initial, y : (y_initial as i8) + 2, hit : false};
                let d = Cords{x : x_initial, y : (y_initial as i8) + 3, hit : false};
                let e = Cords{x : x_initial, y : (y_initial as i8) + 4, hit : false};
                let carrier = Ship{ shiptype : "Carrier".to_string(), cordinates: [a,b,c,d,e], status : false, index : 0};
                return carrier;
            }
            if dir_initial == 'E'{
                let s = convert_char_to_int(&x_initial);

                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : convert_int_to_char(s + 1), y : (y_initial as i8), hit : false};
                let c = Cords{x : convert_int_to_char(s + 2), y : (y_initial as i8), hit : false};
                let d = Cords{x : convert_int_to_char(s + 3), y : (y_initial as i8), hit : false};
                let e = Cords{x : convert_int_to_char(s + 4), y : (y_initial as i8), hit : false};
                let carrier = Ship{ shiptype : "Carrier".to_string(), cordinates: [a,b,c,d,e], status : false, index : 0};
                return carrier;
            }
            if dir_initial == 'W'{
                let s = convert_char_to_int(&x_initial);
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : convert_int_to_char(s - 1), y : (y_initial as i8), hit : false};
                let c = Cords{x : convert_int_to_char(s - 2), y : (y_initial as i8), hit : false};
                let d = Cords{x : convert_int_to_char(s - 3), y : (y_initial as i8), hit : false};
                let e = Cords{x : convert_int_to_char(s - 4), y : (y_initial as i8), hit : false};
                let carrier = Ship{ shiptype : "Carrier".to_string(), cordinates: [a,b,c,d,e], status : false, index : 0};
                return carrier;
            }
        }
    }
}
pub fn check_if_dir_valid(dir : &char) -> bool{
    match dir{
        'N' => return true,
        'S' => return true,
        'E' => return true,
        'W' => return true,
         _ => return {println!("Invalid char: {}", dir); 
            return false},
    };
}
pub fn check_if_y_is_valid(x : &usize) -> bool{
        match x{
        1 => return true,
        2 => return true,
        3 => return true,
        4 => return true,
        5 => return true,
        6 => return true,
        7 => return true,
        8 => return true,
        9 => return true,
        10 => return true,
        _ => return false
    };
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
pub fn check_if_x_is_valid(x : &char) -> bool{
        match x{
            'A' => return true,
            'B' => return true,
            'C' => return true,
            'D' => return true,
            'E' => return true,
            'F' => return true,
            'G' => return true,
            'H' => return true,
            'I' => return true,
            'J' => return true,
            _ => { println!("Invalid char: {}", x); 
            return false},
        };
    }
pub fn convert_char_to_int(x : &char) -> usize{
        match x{
            'A' => return 1,
            'B' => return 2,
            'C' => return 3,
            'D' => return 4,
            'E' => return 5,
            'F' => return 6,
            'G' => return 7,
            'H' => return 8,
            'I' => return 9,
            'J' => return 10,
            _ => return 11
        };
    }
pub fn check_setup(dir: &char, x: &char, y: &usize, size: i8) -> bool{
    if dir == &'N'{
        let test = *y as i8;
        if test - size < 1{
            println!("{}", y);
            println!("Invalids setup");
            return false;
        }
    }
    if dir == &'S'{
        let test = *y as i8;
        if test + size > 10{
            println!("Invalid setup");
            return false;
        }
    }
    if dir == &'W'{
        let test = convert_char_to_int(x) as i8;
        if test - size  < 1{
            println!("Invalid setup");
            return false;
        }
    }
    if dir == &'E'{
        let test = convert_char_to_int(x);
        if test + (size as usize) > 10{
            println!("Invalid setup");
            return false;
        }
    }
    return true;
}
pub fn create_battleship() -> Ship{
    let mut inputx = String::new();
    let mut inputy = String::new();
    let mut input_dir = String::new();
    loop{//Loop till we build a valid ship
        let mut dir_initial = ' ';
        let mut x_initial = ' ';
        let mut y_initial = 0;
        loop{
            println!("Please enter the column for the anchor point of the Battleship(A-J): ");
            io::stdin().read_line(&mut inputx).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmedx = inputx.trim().to_uppercase();
            x_initial = trimmedx.pop().unwrap();
            if check_if_x_is_valid(&x_initial){
                break;
            }   
        }
        loop{
            println!("Please enter the row for the anchor point of the Battleship(1-10): ");
            io::stdin().read_line(&mut inputy).expect("failed to read from stdin");//Example found on stackoverflow
            let trimmedy = inputy.trim();
            let x = trimmedy.len();
            y_initial = trimmedy.chars().next().unwrap().to_digit(10).unwrap();
            if y_initial == 1 && x == 2{
                println!("You entered a value greater than or equal to 10 assuming 10");
                y_initial = 10;
            }
            if check_if_y_is_valid(&(y_initial as usize)){
                break;
            }
        }
        loop{
            println!("Please enter A direction to face the Battleship(N,S,E,W): ");
            io::stdin().read_line(&mut input_dir).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmed_dir = input_dir.trim().to_uppercase();
            dir_initial = trimmed_dir.pop().unwrap();
            if check_if_dir_valid(&dir_initial){
                break;
            }
            else{

            }
        }
        if check_setup(&dir_initial, &x_initial, &(y_initial as usize), 3){
            if dir_initial == 'N'{
                let s = convert_char_to_int(&x_initial);
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : x_initial, y : (y_initial as i8) - 1, hit : false};
                let c = Cords{x : x_initial, y : (y_initial as i8) - 2, hit : false};
                let d = Cords{x : x_initial, y : (y_initial as i8) - 3, hit : false};
                let Battleship = Ship{ shiptype : "Battleship".to_string(), cordinates: [a,b,c,d,Default::default()], status : false, index : 0};
                return Battleship;
            }
            if dir_initial == 'S'{
                let s = convert_char_to_int(&x_initial);

                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : x_initial, y : (y_initial as i8) + 1, hit : false};
                let c = Cords{x : x_initial, y : (y_initial as i8) + 2, hit : false};
                let d = Cords{x : x_initial, y : (y_initial as i8) + 3, hit : false};
                let Battleship = Ship{ shiptype : "Battleship".to_string(), cordinates: [a,b,c,d,Default::default()], status : false, index : 0};
                return Battleship;
            }
            if dir_initial == 'E'{
                let s = convert_char_to_int(&x_initial);

                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : convert_int_to_char(s + 1), y : (y_initial as i8), hit : false};
                let c = Cords{x : convert_int_to_char(s + 2), y : (y_initial as i8), hit : false};
                let d = Cords{x : convert_int_to_char(s + 3), y : (y_initial as i8), hit : false};
                let Battleship = Ship{ shiptype : "Battleship".to_string(), cordinates: [a,b,c,d,Default::default()], status : false, index : 0};
                return Battleship;
            }
            if dir_initial == 'W'{
                let s = convert_char_to_int(&x_initial);
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : convert_int_to_char(s - 1), y : (y_initial as i8), hit : false};
                let c = Cords{x : convert_int_to_char(s - 2), y : (y_initial as i8), hit : false};
                let d = Cords{x : convert_int_to_char(s - 3), y : (y_initial as i8), hit : false};
                let Battleship = Ship{ shiptype : "Battleship".to_string(), cordinates: [a,b,c,d,Default::default()], status : false, index : 0};
                return Battleship;
            }
        }
    }
}
pub fn create_cruiser() -> Ship{
    let mut inputx = String::new();
    let mut inputy = String::new();
    let mut input_dir = String::new();
    loop{//Loop till we build a valid ship
        let mut dir_initial = ' ';
        let mut x_initial = ' ';
        let mut y_initial = 0;
        loop{
            println!("Please enter the column for the anchor point of the Cruiser(A-J): ");
            io::stdin().read_line(&mut inputx).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmedx = inputx.trim().to_uppercase();
            x_initial = trimmedx.pop().unwrap();
            if check_if_x_is_valid(&x_initial){
                break;
            }   
        }
        loop{
            println!("Please enter the row for the anchor point of the Cruiser(1-10): ");
            io::stdin().read_line(&mut inputy).expect("failed to read from stdin");//Example found on stackoverflow
            let trimmedy = inputy.trim();
            let x = trimmedy.len();
            y_initial = trimmedy.chars().next().unwrap().to_digit(10).unwrap();
            if y_initial == 1 && x == 2{
                println!("You entered a value greater than or equal to 10 assuming 10");
                y_initial = 10;
            }
            if check_if_y_is_valid(&(y_initial as usize)){
                break;
            }
        }
        loop{
            println!("Please enter A direction to face the Cruiser(N,S,E,W): ");
            io::stdin().read_line(&mut input_dir).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmed_dir = input_dir.trim().to_uppercase();
            dir_initial = trimmed_dir.pop().unwrap();
            if check_if_dir_valid(&dir_initial){
                break;
            }
            else{

            }
        }
        if check_setup(&dir_initial, &x_initial, &(y_initial as usize), 2){
            if dir_initial == 'N'{
                let s = convert_char_to_int(&x_initial);
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : x_initial, y : (y_initial as i8) - 1, hit : false};
                let c = Cords{x : x_initial, y : (y_initial as i8) - 2, hit : false};
                let Cruiser = Ship{ shiptype : "Cruiser".to_string(), cordinates: [a,b,c,Default::default(),Default::default()], status : false, index : 0};
                return Cruiser;
            }
            if dir_initial == 'S'{
                let s = convert_char_to_int(&x_initial);

                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : x_initial, y : (y_initial as i8) + 1, hit : false};
                let c = Cords{x : x_initial, y : (y_initial as i8) + 2, hit : false};
                let Cruiser = Ship{ shiptype : "Cruiser".to_string(), cordinates: [a,b,c,Default::default(),Default::default()], status : false, index : 0};
                return Cruiser;
            }
            if dir_initial == 'E'{
                let s = convert_char_to_int(&x_initial);

                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : convert_int_to_char(s + 1), y : (y_initial as i8), hit : false};
                let c = Cords{x : convert_int_to_char(s + 2), y : (y_initial as i8), hit : false};
                let Cruiser = Ship{ shiptype : "Cruiser".to_string(), cordinates: [a,b,c,Default::default(),Default::default()], status : false, index : 0};
                return Cruiser;
            }
            if dir_initial == 'W'{
                let s = convert_char_to_int(&x_initial);
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : convert_int_to_char(s - 1), y : (y_initial as i8), hit : false};
                let c = Cords{x : convert_int_to_char(s - 2), y : (y_initial as i8), hit : false};
                let Cruiser = Ship{ shiptype : "Cruiser".to_string(), cordinates: [a,b,c,Default::default(),Default::default()], status : false, index : 0};
                return Cruiser;
            }
        }
    }
}
pub fn create_submarine() -> Ship{
    let mut inputx = String::new();
    let mut inputy = String::new();
    let mut input_dir = String::new();
    loop{//Loop till we build a valid ship
        let mut dir_initial = ' ';
        let mut x_initial = ' ';
        let mut y_initial = 0;
        loop{
            println!("Please enter the column for the anchor point of the sub(A-J): ");
            io::stdin().read_line(&mut inputx).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmedx = inputx.trim().to_uppercase();
            x_initial = trimmedx.pop().unwrap();
            println!("x: {}", x_initial);
            if check_if_x_is_valid(&x_initial){
                break;
            }   
        }
        loop{
            println!("Please enter the row for the anchor point of the sub(1-10): ");
            io::stdin().read_line(&mut inputy).expect("failed to read from stdin");//Example found on stackoverflow
            let trimmedy = inputy.trim();
            let x = trimmedy.len();
            y_initial = trimmedy.chars().next().unwrap().to_digit(10).unwrap();
            if y_initial == 1 && x == 2{
                println!("You entered a value greater than or equal to 10 assuming 10");
                y_initial = 10;
            }
            println!("y: {}",y_initial );
            if check_if_y_is_valid(&(y_initial as usize)){
                break;
            }
        }
        loop{
            println!("Please enter A direction to face the sub(N,S,E,W): ");
            io::stdin().read_line(&mut input_dir).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmed_dir = input_dir.trim().to_uppercase();
            dir_initial = trimmed_dir.pop().unwrap();
            println!("Direction: {}",input_dir );
            if check_if_dir_valid(&dir_initial){
                break;
            }
            else{

            }
        }
        if check_setup(&dir_initial, &x_initial, &(y_initial as usize), 2){
            if dir_initial == 'N'{
                let s = convert_char_to_int(&x_initial);
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : x_initial, y : (y_initial as i8) - 1, hit : false};
                let c = Cords{x : x_initial, y : (y_initial as i8) - 2, hit : false};
                let sub = Ship{ shiptype : "sub".to_string(), cordinates: [a,b,c,Default::default(),Default::default()], status : false, index : 0};
                return sub;
            }
            if dir_initial == 'S'{
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : x_initial, y : (y_initial as i8) + 1, hit : false};
                let c = Cords{x : x_initial, y : (y_initial as i8) + 2, hit : false};
                let sub = Ship{ shiptype : "sub".to_string(), cordinates: [a,b,c,Default::default(),Default::default()], status : false, index : 0};
                return sub;
            }
            if dir_initial == 'E'{
                let s = convert_char_to_int(&x_initial);

                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : convert_int_to_char(s + 1), y : (y_initial as i8), hit : false};
                let c = Cords{x : convert_int_to_char(s + 2), y : (y_initial as i8), hit : false};
                let sub = Ship{ shiptype : "sub".to_string(), cordinates: [a,b,c,Default::default(),Default::default()], status : false, index : 0};
                return sub;
            }
            if dir_initial == 'W'{
                let s = convert_char_to_int(&x_initial);
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : convert_int_to_char(s - 1), y : (y_initial as i8), hit : false};
                let c = Cords{x : convert_int_to_char(s - 2), y : (y_initial as i8), hit : false};
                let sub = Ship{ shiptype : "sub".to_string(), cordinates: [a,b,c,Default::default(),Default::default()], status : false, index : 0};
                return sub;
            }
        }
    }
}
pub fn create_destroyer() -> Ship{
    let mut inputx = String::new();
    let mut inputy = String::new();
    let mut input_dir = String::new();
    loop{//Loop till we build a valid ship
        let mut dir_initial = ' ';
        let mut x_initial = ' ';
        let mut y_initial = 0;
        loop{
            println!("Please enter the column for the anchor point of the Destroyer(A-J): ");
            io::stdin().read_line(&mut inputx).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmedx = inputx.trim().to_uppercase();
            x_initial = trimmedx.pop().unwrap();
            if check_if_x_is_valid(&x_initial){
                break;
            }   
        }
        loop{
            println!("Please enter the row for the anchor point of the Destroyer(1-10): ");
            io::stdin().read_line(&mut inputy).expect("failed to read from stdin");//Example found on stackoverflow
            let trimmedy = inputy.trim();
            let x = trimmedy.len();
            y_initial = trimmedy.chars().next().unwrap().to_digit(10).unwrap();
            if y_initial == 1 && x == 2{
                println!("You entered a value greater than or equal to 10 assuming 10");
                y_initial = 10;
            }
            if check_if_y_is_valid(&(y_initial as usize)){
                break;
            }
        }
        loop{
            println!("Please enter A direction to face the Destroyer(N,S,E,W): ");
            io::stdin().read_line(&mut input_dir).expect("failed to read from stdin");//Example found on stackoverflow
            let mut trimmed_dir = input_dir.trim().to_uppercase();
            dir_initial = trimmed_dir.pop().unwrap();
            if check_if_dir_valid(&dir_initial){
                break;
            }
            else{

            }
        }
        if check_setup(&dir_initial, &x_initial, &(y_initial as usize), 1){
            if dir_initial == 'N'{
                let s = convert_char_to_int(&x_initial);
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : x_initial, y : (y_initial as i8) - 1, hit : false};
                let mut Destroyer = Ship{ shiptype : "Destroyer".to_string(), cordinates: [a,b,Default::default(),Default::default(),Default::default()], status : false, index : 0};
                return Destroyer;
            }
            if dir_initial == 'S'{
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : x_initial, y : (y_initial as i8) + 1, hit : false};
                let mut Destroyer = Ship{ shiptype : "Destroyer".to_string(), cordinates: [a,b,Default::default(),Default::default(),Default::default()], status : false, index : 0};
                return Destroyer;
            }
            if dir_initial == 'E'{
                let s = convert_char_to_int(&x_initial);

                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : convert_int_to_char(s + 1), y : (y_initial as i8), hit : false};
                let mut Destroyer = Ship{ shiptype : "Destroyer".to_string(), cordinates: [a,b,Default::default(),Default::default(),Default::default()], status : false, index : 0};
                return Destroyer;
            }
            if dir_initial == 'W'{
                let s = convert_char_to_int(&x_initial);
                let a = Cords{x : x_initial, y : (y_initial as i8), hit : false};
                let b = Cords{x : convert_int_to_char(s - 1), y : (y_initial as i8), hit : false};
                let mut Destroyer = Ship{ shiptype : "Destroyer".to_string(), cordinates: [a,b,Default::default(),Default::default(),Default::default()], status : false, index : 0};
                return Destroyer;
            }
        }
    }
}