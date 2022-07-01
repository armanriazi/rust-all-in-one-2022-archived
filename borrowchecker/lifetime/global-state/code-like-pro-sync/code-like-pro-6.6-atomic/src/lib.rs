
#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::atomic::{AtomicI32, Ordering};
    lazy_static! {
        static ref COUNT: AtomicI32 = AtomicI32::new(0);
    }
    #[test]
    fn test_count() {
        COUNT.fetch_add(1, Ordering::SeqCst);
        println!("{:?}",1);
        test_count2();
    }
    #[test]
    fn test_count2() {
        COUNT.fetch_add(1, Ordering::SeqCst);
        println!("{:?}",2);
    }
    #[test]
    fn test_count3() {
        COUNT.fetch_add(1, Ordering::SeqCst);
        println!("{:?}",3);
    }
}
    