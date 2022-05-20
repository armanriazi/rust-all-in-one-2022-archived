fn main() {
    let (.., x, y) = (0, 1, ..);
    print!("{}", b"066"[y][x]);
}
/*
54
This question demonstrates two different meanings of ...

In expression position, .. is the syntax for constructing various types of ranges. Here the expression (0, 1, ..) is a tuple with three elements, the third one having type RangeFull.

On the other hand in a pattern, .. is used to mean "any number of elements". So the pattern (.., x, y) matches a tuple with 2 or more elements, binding the second-last one to x and the last one to y.

Coming out of the first line of main, we have x = 1 and y = (..). 

Important: Thus the value printed is going to be b"066"[..][1].

The expression b"066" is a byte-string literal of type &'static [u8; 3] containing the three ASCII bytes b'0', b'6', b'6'.

When we slice the byte-string with RangeFull we get a dynamically sized slice [u8] of length 3. Next we access element 1 of the slice, which is the byte b'6' of type u8. When printed, we see the decimal representation of the byte value of the ASCII digit 6, which is the number 54.
*/