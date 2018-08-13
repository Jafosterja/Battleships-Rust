use ship::Cords;
use ship::Ship;
#[test]
fn test_ship(){
    let a = Cords{x : 'A', y : 1, hit : false};
    let b = Cords{x : 'A', y : 2, hit : false};
    let c = Cords{x : 'A', y : 3, hit : false};
    let mut test_ship = Ship{ shiptype : "Sub".to_string(), cordinates: [Default::default(),Default::default(),Default::default(),Default::default(),Default::default()], status : false, index : 0};
    test_ship.add_cords(a);
    test_ship.add_cords(b);
    test_ship.add_cords(c);
    assert_eq!(test_ship.get_type(), &"Sub".to_string());
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
    
}