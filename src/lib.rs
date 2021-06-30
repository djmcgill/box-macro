/// `box_!(x)` => `Box::new(x)`
#[macro_export]
macro_rules! box_ {
    ($x:expr) => {
        Box::new($x)
    };
}

/// `b0x_!(x)` => `Box::new(x)`
#[macro_export]
macro_rules! b0x {
    ($x:expr) => {
        Box::new($x)
    };
}

/// `bx_!(x)` => `Box::new(x)`
#[macro_export]
macro_rules! bx {
    ($x:expr) => {
        Box::new($x)
    };
}

/// `b_!(x)` => `Box::new(x)`
#[macro_export]
macro_rules! b {
    ($x:expr) => {
        Box::new($x)
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn box_test() {
        let _: Box<()> = box_!(());
    }

    #[test]
    fn b0x_test() {
        let _: Box<()> = b0x!(());
    }

    #[test]
    fn bx_test() {
        let _: Box<()> = bx!(());
    }

    #[test]
    fn b_test() {
        let _: Box<()> = b!(());
    }
}
