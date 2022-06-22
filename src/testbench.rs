mod tests {

    #[test]
    fn sign_conversion_test() {
        use crate::integer::{signed_to_unsigned, unsigned_to_signed};

        for signed in -100..100 {
            assert_eq!(signed, unsigned_to_signed(signed_to_unsigned(signed)));
        }

        for unsigned in 0..100 {
            assert_eq!(unsigned, signed_to_unsigned(unsigned_to_signed(unsigned)));
        }

    }

    #[test]
    fn vector_test() {
        use crate::HSerde;

        assert_eq!(
            Vec::<u32>::new(),
            Vec::<u32>::from_bytes(&(Vec::<u32>::new()).to_bytes(), 0).unwrap()
        );
        assert_eq!(
            Vec::<u8>::new(),
            Vec::<u8>::from_bytes(&(Vec::<u8>::new()).to_bytes(), 0).unwrap()
        );
        assert_eq!(
            Vec::<String>::new(),
            Vec::<String>::from_bytes(&(Vec::<String>::new()).to_bytes(), 0).unwrap()
        );
        assert_eq!(
            vec![vec![], vec![], vec![0u32], vec![1, 2, 3], vec![], vec![]],
            Vec::<Vec<u32>>::from_bytes(&(vec![vec![], vec![], vec![0u32], vec![1, 2, 3], vec![], vec![]]).to_bytes(), 0).unwrap()
        );
        assert_eq!(
            vec![vec![], vec![], vec![0u8], vec![1, 2, 3], vec![], vec![]],
            Vec::<Vec<u8>>::from_bytes(&(vec![vec![], vec![], vec![0u8], vec![1, 2, 3], vec![], vec![]]).to_bytes(), 0).unwrap()
        );
        assert_eq!(
            vec![vec![], vec![], vec!["".to_string()], vec!["a".to_string(), "".to_string(), "abc".to_string()], vec![], vec!["abcdef".to_string()]],
            Vec::<Vec<String>>::from_bytes(&(vec![vec![], vec![], vec!["".to_string()], vec!["a".to_string(), "".to_string(), "abc".to_string()], vec![], vec!["abcdef".to_string()]]).to_bytes(), 0).unwrap()
        );
        assert_eq!(
            vec![vec![], vec![true, true], vec![false, true], vec![], vec![true, false]],
            Vec::<Vec<bool>>::from_bytes(&(vec![vec![], vec![true, true], vec![false, true], vec![], vec![true, false]]).to_bytes(), 0).unwrap()
        );
        assert_eq!(
            (-1000..1000i32).collect::<Vec<i32>>(),
            Vec::<i32>::from_bytes(&((-1000..1000i32).collect::<Vec<i32>>()).to_bytes(), 0).unwrap()
        );
        assert_eq!(
            (0..1000u32).collect::<Vec<u32>>(),
            Vec::<u32>::from_bytes(&((0..1000u32).collect::<Vec<u32>>()).to_bytes(), 0).unwrap()
        );
    }

    #[test]
    fn integer_test() {
        use crate::HSerde;

        for n in 0..255u8 {
            assert_eq!(n, u8::from_bytes(&n.to_bytes(), 0).unwrap());
        }

        let mut n: u32 = 1;

        for _ in 0..30 {
            assert_eq!(n, u32::from_bytes(&n.to_bytes(), 0).unwrap());
            assert_eq!(n as i32, i32::from_bytes(&(n as i32).to_bytes(), 0).unwrap());
            assert_eq!(-(n as i32), i32::from_bytes(&(-(n as i32)).to_bytes(), 0).unwrap());

            n *= 2;
        }

    }

    #[test]
    fn boolean_test() {
        use crate::HSerde;

        assert_eq!(true, bool::from_bytes(&true.to_bytes(), 0).unwrap());
        assert_eq!(false, bool::from_bytes(&false.to_bytes(), 0).unwrap());
    }

    #[test]
    fn string_test() {
        use crate::HSerde;

        assert_eq!("".to_string(), String::from_bytes(&("".to_string()).to_bytes(), 0).unwrap());
        assert_eq!("a".to_string(), String::from_bytes(&("a".to_string()).to_bytes(), 0).unwrap());
        assert_eq!("abcdefg".to_string(), String::from_bytes(&("abcdefg".to_string()).to_bytes(), 0).unwrap());
    }
}