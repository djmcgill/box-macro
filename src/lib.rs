/// `box_!(x)` => `Box::new(x)`
#[macro_export]
macro_rules! box_ {
    ($x:expr) => {
        Box::new($x)
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let _: Box<()> = box_!(());
    }
}
