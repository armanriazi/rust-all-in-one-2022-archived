
static N:i32 = 15;


pub fn get_statx()-> String{
    "Arman".to_string()
}

pub fn get_staty<'a>()->&'a str{
  &"Arman"
}

pub fn get_stat()->&'static i32{
    &N
}

pub fn stat_str()->&'static str{
    "hello"
}

static mut M:i32 = 20;

pub fn add_stat(n:i32)->i32 {
    unsafe {
        M += n;
        M
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let x = get_statx();
        assert_eq!(*x, "Arman".to_string());

        let y = get_staty();
        assert_eq!(y, "Arman");

        let n = get_stat();
        assert_eq!(*n, 15);

        let s = stat_str();
        assert_eq!(s,"hello");

        let m = add_stat(5);
        assert_eq!(m,25);
    }
}
