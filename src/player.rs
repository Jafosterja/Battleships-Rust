use ship::Cords;
use ship::Ship;

#[derive(Clone)]
pub struct Player{
    pub myships : [Ship; 5],
    pub alive : bool,
    pub a_ships : i8,
    pub index : usize

}
pub fn check_if_alive(player : Player) -> bool{
    let mut result = true;
    for ships in 0..5{
        let test = player.myships[ships].clone();
        if test.check_if_dead() == false{
            result = false;
            return result;
        }
    }
    return result;
}
impl Default for Player {
    fn default() -> Player {
        Player {
            myships : [Default::default(),Default::default(),Default::default(),Default::default(),Default::default()],
            alive : true,
            a_ships : 5,
            index : 0,
        }
    }
}
impl Player{
    pub fn check_status(&self) -> bool{
        if self.alive == true{
            return true;
        }
        return false;
    }
    pub fn check_cords(mut self, x : char, y : usize) -> Player{
        let check = Cords{x : x, y : (y as i8), hit : false};
        for ships in 0..5{
            let mut test = self.myships[ships].clone();
            test = test.check_if_hit(check);
            self.myships[ships] = test;
            }
        let self_clone = self.clone();
        self.alive = check_if_alive(self_clone);
        return self
    }
    pub fn check_if_ship_fits(&self, ship : &Ship) -> bool{
        for x in 0..5{//Index for ships in player struct
            for y in 0..5{//index for cords for the ship passed in
                for j in 0..5{//index for cords in each ship for the player class
                    if ship.cordinates[y].x == self.myships[x].cordinates[j].x && ship.cordinates[y].y == self.myships[x].cordinates[j].y{
                        return false;
                    }
                }
            }    
        }
        return true;
    }
    pub fn add_ship(mut self, ship : Ship) -> Player{
        self.myships[self.index] = ship;
        self.index = self.index + 1;
        return self;
    }
}

