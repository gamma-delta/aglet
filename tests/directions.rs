use aglet::{Direction4, Direction8};

#[test]
fn test_spin() {
  for dir4 in Direction4::DIRECTIONS {
    assert_eq!(dir4, dir4.rotate_by(0));
    assert_eq!(dir4, dir4.rotate_by(4));
  }

  for dir8 in Direction8::DIRECTIONS {
    assert_eq!(dir8, dir8.rotate_by(0));
    assert_eq!(dir8, dir8.rotate_by(8));
  }
}
