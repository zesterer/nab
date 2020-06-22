#[macro_export]
macro_rules! nab {
    ($i:expr, $p:pat => $o:expr) => {
        if let $p = $i {
            ::std::option::Option::Some($o)
        } else {
            ::std::option::Option::None
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Copy, Clone)]
    enum Foo {
        A(i32, bool),
        B { x: bool, y: u32 },
    }
    
    #[test]
    fn basic() {
        let input = Foo::A(42, true);
        
        let r = nab!(input, Foo::A(x, y) => (x, y));
        assert_eq!(r, Some((42, true)));
        
        let r = nab!(input, Foo::B { x, y } => (x, y));
        assert_eq!(r, None);
    }
}
