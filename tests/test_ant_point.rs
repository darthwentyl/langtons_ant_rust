use langtons_ant_rust::ant_point::AntPoint;

#[test]
fn ant_point_construct_by_new() {
    let ant_point = AntPoint::new(11, 22);
    assert_eq!(ant_point.x(), 11);
    assert_eq!(ant_point.y(), 22);
}

#[test]
fn ant_point_set_x() {
    let mut ant_point = AntPoint::new(11, 22);
    ant_point.set_x(33);
    assert_eq!(ant_point.x(), 33);
    assert_eq!(ant_point.y(), 22);
}

#[test]
fn ant_point_set_y() {
    let mut ant_point = AntPoint::new(11, 22);
    ant_point.set_y(44);
    assert_eq!(ant_point.x(), 11);
    assert_eq!(ant_point.y(), 44);
}

#[test]
fn ant_point_move_1_x() {
    let mut ant_point = AntPoint::new(11, 22);
    ant_point.move_x(1);
    assert_eq!(ant_point.x(), 12);
    assert_eq!(ant_point.y(), 22);
}

#[test]
fn ant_point_move_minus_1_x() {
    let mut ant_point = AntPoint::new(11, 22);
    ant_point.move_x(-1);
    assert_eq!(ant_point.x(), 10);
    assert_eq!(ant_point.y(), 22);
}

#[test]
fn ant_point_move_1_y() {
    let mut ant_point = AntPoint::new(11, 22);
    ant_point.move_y(1);
    assert_eq!(ant_point.x(), 11);
    assert_eq!(ant_point.y(), 23);
}

#[test]
fn ant_point_move_minus_1_y() {
    let mut ant_point = AntPoint::new(11, 22);
    ant_point.move_y(-1);
    assert_eq!(ant_point.x(), 11);
    assert_eq!(ant_point.y(), 21);
}