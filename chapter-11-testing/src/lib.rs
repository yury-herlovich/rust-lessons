#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn area(&self) -> u32 {
        self.width * self.height
    }

    #[allow(dead_code)]
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    #[allow(dead_code)]
    fn longest_side(&self) -> Result<u32, String> {
        if self.is_square() {
            Err(String::from("Rectangle is square"))
        } else {
            if self.width > self.height {
                Ok(self.width)
            } else {
                Ok(self.height)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn failed_test() {
    //     assert_eq!(2 + 2, 2);
    // }

    #[test]
    fn should_calculate_area() {
        let rec = Rectangle {
            width: 3,
            height: 2,
        };

        assert_eq!(rec.area(), 6); // using assert_eq
        assert!(rec.area() == 6); // using assert(true)
        assert_ne!(rec.area(), 10);
    }

    #[test]
    fn rec_is_square() {
        let rec = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(rec.is_square())
    }

    #[test]
    fn res_is_not_square() {
        let rec = Rectangle {
            width: 5,
            height: 10,
        };

        assert!(!rec.is_square())
    }

    #[test]
    fn should_return_longest_side() {
        let rec = Rectangle {
            width: 6,
            height: 9,
        };

        assert!(rec.longest_side().unwrap() == 9);
    }

    #[test]
    #[should_panic] // test will fail without this line
    fn should_longest_side_fail_if_square() {
        let rec = Rectangle {
            width: 7,
            height: 7,
        };

        rec.longest_side().unwrap();
    }
}
