use ship::Cords;
use ship::Ship;

#[derive(Clone)]
pub struct Player{
    pub myships : [Ship; 5],
    pub alive : bool,
    pub a_ships : i8,

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
}