use langtons_ant_rust::term_color::TermColor;

#[test]
fn term_color_comparison_the_same() {
    let color_one = TermColor::new(1, 2, 3);
    let color_two = TermColor::new(1, 2, 3);
    assert!(color_one == color_two);
}

#[test]
fn term_color_comparison_diff() {
    let color_one = TermColor::new(1, 2, 3);
    let color_two = TermColor::new(11, 22, 33);
    assert!(color_one != color_two);
}

#[test]
fn term_color_set_get_r() {
    let mut color = TermColor::new(1, 2, 3);
    color.set_r(11);
    assert_eq!(color.r(), 11);
    assert_eq!(color.g(), 2);
    assert_eq!(color.b(), 3);
}

#[test]
fn term_color_set_get_g() {
    let mut color = TermColor::new(1, 2, 3);
    color.set_g(22);
    assert_eq!(color.r(), 1);
    assert_eq!(color.g(), 22);
    assert_eq!(color.b(), 3);
}

#[test]
fn term_color_set_get_b() {
    let mut color = TermColor::new(1, 2, 3);
    color.set_b(33);
    assert_eq!(color.r(), 1);
    assert_eq!(color.g(), 2);
    assert_eq!(color.b(), 33);
}