// The derive implements <BookFormat> == <BookFormat> comparisons
#[derive(PartialEq)]
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}
#[derive(PartialEq)]
struct Book {
    isbn: i32,
    format: BookFormat,
}

// Implement <Book> == <BookFormat> comparisons
impl PartialEq<BookFormat> for Book {
    fn eq(&self, other: &BookFormat) -> bool {
        self.format == *other
    }
}

// Implement <BookFormat> == <Book> comparisons
impl PartialEq<Book> for BookFormat {
    fn eq(&self, other: &Book) -> bool {
        *self == other.format
    }
}
fn equal<T: std::ops::PartialEq<Output = bool>>(i: T, j: T) -> bool{
    i.eq(&j)
}


fn main(){}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test(){
        let b1 = Book { isbn: 3, format: BookFormat::Paperback };
        let b2 = Book { isbn: 4, format: BookFormat::Paperback };
        let b3 = Book { isbn: 3, format: BookFormat::Paperback };

        assert!(b1 == BookFormat::Paperback);
        assert!(BookFormat::Paperback == b2);
        assert!(BookFormat::Ebook != b1);       
        assert!(b1 == b3);       
        assert_eq!(b1.eq(&b2),false);       
    }
}