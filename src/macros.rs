macro_rules! vecdeq {
    () => {
        <::std::vec::Vec<_> as Into<::std::collections::VecDeque<_>>>::into(::std::vec![])
    };
    ($elem:expr; $n:expr) => {
        <::std::vec::Vec<_> as Into<::std::collections::VecDeque<_>>>::into(::std::vec![$elem; $n])
    };
    ($($x:expr),+ $(,)?) => {
        <::std::vec::Vec<_> as Into<::std::collections::VecDeque<_>>>::into(::std::vec![$($x),+])
    };
}

macro_rules! unsafe_unreachable {
    () => {
        unsafe {
            ::std::hint::unreachable_unchecked()
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_unreachable_unchecked() {
        match 17 {
            0..4 => println!("0..4"),
            17 => println!("match!"),
            _ => unsafe_unreachable!()
        }
    }
}
