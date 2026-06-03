use langtons_ant_rust::ant_point::AntPoint;
use langtons_ant_rust::ant::Ant;
use langtons_ant_rust::ant_direction::EAntDirection;
use langtons_ant_rust::ant_board_color::EAntBoardColor;

#[test]
fn ant_construct_by_new() {
    let start_pos = AntPoint::new(11, 22);
    let start_direction = EAntDirection::UP;
    let ant = Ant::new(start_pos, start_direction);

    assert_eq!(ant.get_curr_pos().x(), 11);
    assert_eq!(ant.get_curr_pos().y(), 22);
    assert!(ant.get_curr_direction() == EAntDirection::UP);
}

#[test]
fn ant_curr_right_color_green() {
    let start_pos = AntPoint::new(5, 5);
    let start_direction = EAntDirection::RIGHT;
    let mut ant = Ant::new(start_pos, start_direction);
    ant.set_position(EAntBoardColor::GREEN);

    assert_eq!(ant.get_curr_pos().x(), 5);
    assert_eq!(ant.get_curr_pos().y(), 6);
    assert!(ant.get_curr_direction() == EAntDirection::DOWN);
}

#[test]
fn ant_curr_down_color_green() {
    let start_pos = AntPoint::new(5, 6);
    let start_direction = EAntDirection::DOWN;
    let mut ant = Ant::new(start_pos, start_direction);
    ant.set_position(EAntBoardColor::GREEN);

    assert_eq!(ant.get_curr_pos().x(), 4);
    assert_eq!(ant.get_curr_pos().y(), 6);
    assert!(ant.get_curr_direction() == EAntDirection::LEFT);
}

#[test]
fn ant_curr_left_color_green() {
    let start_pos = AntPoint::new(4, 6);
    let start_direction = EAntDirection::LEFT;
    let mut ant = Ant::new(start_pos, start_direction);
    ant.set_position(EAntBoardColor::GREEN);

    assert_eq!(ant.get_curr_pos().x(), 4);
    assert_eq!(ant.get_curr_pos().y(), 5);
    assert!(ant.get_curr_direction() == EAntDirection::UP);
}

#[test]
fn ant_curr_up_color_green() {
    let start_pos = AntPoint::new(4, 5);
    let start_direction = EAntDirection::UP;
    let mut ant = Ant::new(start_pos, start_direction);
    ant.set_position(EAntBoardColor::GREEN);

    assert_eq!(ant.get_curr_pos().x(), 5);
    assert_eq!(ant.get_curr_pos().y(), 5);
    assert!(ant.get_curr_direction() == EAntDirection::RIGHT);
}

#[test]
fn ant_curr_right_color_red() {
    let start_pos = AntPoint::new(5, 5);
    let start_direction = EAntDirection::RIGHT;
    let mut ant = Ant::new(start_pos, start_direction);
    ant.set_position(EAntBoardColor::RED);

    assert_eq!(ant.get_curr_pos().x(), 5);
    assert_eq!(ant.get_curr_pos().y(), 4);
    assert!(ant.get_curr_direction() == EAntDirection::UP);
}

#[test]
fn ant_curr_up_color_red() {
    let start_pos = AntPoint::new(5, 4);
    let start_direction = EAntDirection::UP;
    let mut ant = Ant::new(start_pos, start_direction);
    ant.set_position(EAntBoardColor::RED);

    assert_eq!(ant.get_curr_pos().x(), 4);
    assert_eq!(ant.get_curr_pos().y(), 4);
    assert!(ant.get_curr_direction() == EAntDirection::LEFT);
}

#[test]
fn ant_curr_left_color_red() {
    let start_pos = AntPoint::new(4, 4);
    let start_direction = EAntDirection::LEFT;
    let mut ant = Ant::new(start_pos, start_direction);
    ant.set_position(EAntBoardColor::RED);

    assert_eq!(ant.get_curr_pos().x(), 4);
    assert_eq!(ant.get_curr_pos().y(), 5);
    assert!(ant.get_curr_direction() == EAntDirection::DOWN);
}

#[test]
fn ant_curr_down_color_red() {
    let start_pos = AntPoint::new(4, 5);
    let start_direction = EAntDirection::DOWN;
    let mut ant = Ant::new(start_pos, start_direction);
    ant.set_position(EAntBoardColor::RED);

    assert_eq!(ant.get_curr_pos().x(), 5);
    assert_eq!(ant.get_curr_pos().y(), 5);
    assert!(ant.get_curr_direction() == EAntDirection::RIGHT);
}