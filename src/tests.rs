use ship::Cords;
use ship::Ship;
use player::Player;
#[test]
fn test_ship(){
    let a = Cords{x : 'A', y : 1, hit : false};
    let b = Cords{x : 'A', y : 2, hit : false};
    let c = Cords{x : 'A', y : 3, hit : false};
    let mut test_ship = Ship{ shiptype : "sub".to_string(), cordinates: [Default::default(),Default::default(),Default::default(),Default::default(),Default::default()], status : false, index : 0};
    test_ship.add_cords(a);
    test_ship.add_cords(b);
    test_ship.add_cords(c);
    assert_eq!(test_ship.get_type(), &"sub".to_string());
    test_ship = test_ship.check_if_hit(b);
    test_ship = test_ship.check_if_hit(a);
    let mut test = test_ship.clone();
    assert_eq!(test.check_if_dead(), false);
    test_ship = test_ship.check_if_hit(c);
    test = test_ship.clone();
    assert_eq!(test.check_if_dead(), true);
}
#[test]
fn test_player(){
    let mut player = build_test_player();
    player = player.check_cords('A', 1);
    player = player.check_cords('A', 2);
    player = player.check_cords('A', 3);
    player = player.check_cords('A', 4);
    player = player.check_cords('A', 5);
    assert_eq!(player.check_status(), true);
    assert_eq!(player.myships[0].status, false);
    player = player.check_cords('B', 1);
    player = player.check_cords('B', 2);
    player = player.check_cords('B', 3);
    player = player.check_cords('B', 4);
    assert_eq!(player.check_status(), true);
    assert_eq!(player.myships[1].status, false);
    player = player.check_cords('C', 1);
    player = player.check_cords('C', 2);
    player = player.check_cords('C', 3);
    assert_eq!(player.check_status(), true);
    assert_eq!(player.myships[2].status, false);
    player = player.check_cords('D', 1);
    player = player.check_cords('D', 2);
    player = player.check_cords('D', 3);
    assert_eq!(player.check_status(), true);
    assert_eq!(player.myships[2].status, false);
    player = player.check_cords('E', 1);
    player = player.check_cords('E', 2);
    assert_eq!(player.check_status(), false);
    assert_eq!(player.myships[2].status, false);
}

pub fn build_test_player() -> Player{
    let mut player : Player = Default::default();
    player = player.add_ship(build_test_carr());
    player = player.add_ship(build_test_bs());
    player = player.add_ship(build_test_cruiser());
    player = player.add_ship(build_test_sub());
    player = player.add_ship(build_test_dest());
    return player;
}
pub fn build_test_carr() -> Ship{
    let a = Cords{x : 'A', y : 1, hit : false};
    let b = Cords{x : 'A', y : 2, hit : false};
    let c = Cords{x : 'A', y : 3, hit : false};
    let d = Cords{x : 'A', y : 4, hit : false};
    let e = Cords{x : 'A', y : 5, hit : false};
    return Ship{ shiptype : "Carrier".to_string(), cordinates: [a,b,c,d,e], status : false, index : 0};
}
pub fn build_test_bs() -> Ship{
    let a = Cords{x : 'B', y : 1, hit : false};
    let b = Cords{x : 'B', y : 2, hit : false};
    let c = Cords{x : 'B', y : 3, hit : false};
    let d = Cords{x : 'B', y : 4, hit : false};
    return Ship{ shiptype : "Battleship".to_string(), cordinates: [a,b,c,d, Default::default()], status : false, index : 0};
}
pub fn build_test_cruiser() -> Ship{
    let a = Cords{x : 'C', y : 1, hit : false};
    let b = Cords{x : 'C', y : 2, hit : false};
    let c = Cords{x : 'C', y : 3, hit : false};
    return Ship{ shiptype : "Cruiser".to_string(), cordinates: [a,b,c,Default::default(),Default::default()], status : false, index : 0};
}
pub fn build_test_sub() -> Ship{
    let a = Cords{x : 'D', y : 1, hit : false};
    let b = Cords{x : 'D', y : 2, hit : false};
    let c = Cords{x : 'D', y : 3, hit : false};
    return Ship{ shiptype : "sub".to_string(), cordinates: [a,b,c,Default::default(),Default::default()], status : false, index : 0};
}
pub fn build_test_dest() -> Ship{//build a test destroyer
    let a = Cords{x : 'E', y : 1, hit : false};
    let b = Cords{x : 'E', y : 2, hit : false};
    return Ship{ shiptype : "sub".to_string(), cordinates: [a,b,Default::default(),Default::default(),Default::default()], status : false, index : 0};
}