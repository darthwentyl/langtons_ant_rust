use langtons_ant_rust::ant_point::AntPoint;

#[test]
fn point_constructor_by_new() {
    let ant_point = AntPoint::new(11, 22);
    assert_eq!(ant_point.x(), 11);
    assert_eq!(ant_point.y(), 22);
}

