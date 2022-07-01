
#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::atomic::{AtomicI32, Ordering};
    use log::{debug, error, log_enabled, info, Level};    
    extern crate log;
    lazy_static! {
        static ref COUNT: AtomicI32 = AtomicI32::new(0);
    }
    fn init() {
        
        let _ = env_logger::builder().is_test(true).try_init();        
        if log_enabled!(Level::Info) {            
            info!("Welcome to env_logger");
        }
    }

    #[test]
    fn it_adds_one() {
        init();
        info!("can log from the test too");        
    }
    #[test]
    fn test_count() {
        COUNT.fetch_add(1, Ordering::SeqCst);
        info!("test_count");        
        test_count2();
    }
    #[test]
    fn test_count2() {
        COUNT.fetch_add(1, Ordering::SeqCst);
        info!("test_count2");        
    }
    #[test]
    fn test_count3() {
        COUNT.fetch_add(1, Ordering::SeqCst);
        info!("test_count3");        
    }
}


extern crate log;
fn main(){}    