use chapter_11_testing::Rectangle;

mod common;

#[test]
fn rectangle_is_square() {
    common::setup();

    let rec = Rectangle {
        width: 5,
        height: 5,
    };

    assert_eq!(rec.is_square(), true);
}
