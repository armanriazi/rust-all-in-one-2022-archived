macro_rules! m {
    (==>) => { print!("1"); };
    (= = >) => { print!("2"); };
    (== >) => { print!("3"); };
    (= =>) => { print!("4"); };
    (<<=) => { print!("5"); };
    (=>>) => { print!("6"); };
    (=<<) => { print!("7"); };
    (= <<) => { print!("8"); };
    (= <) => { print!("9"); };
    (= >) => { print!("10"); };
}

fn main() {
    m!(==>);
    m!(= = >);
    m!(== >);
    m!(= =>);
    m!(<<=);
    m!(=>>);
    m!(=<<);
    m!(= <<);
    m!(= <);
    m!(= >);
}
/*
1214
Adjacent punctuation characters in the input pattern of a macro_rules! macro are grouped according to how those characters are used by native Rust tokens.

This page contains a list of the single-character and multi-character punctuation tokens involved in the Rust grammar.

As one example from that list, <<= is a single token because the Rust grammar uses that sequence of characters to mean left shift assignment. Thus a macro_rules! input rule containing <<= would only match if all three characters <<= are written consecutively without spaces in the invocation.

But for example =<< is not a native token in the Rust grammar. 
The parser of macro_rules! will decompose this into Rust tokens according to a greedy process. =< is also not a native token, so first we would need to match a = by itself. 
Then << is a native token. Writing =<< in a macro rule behaves exactly the same as writing = <<.

Now let's decompose the rules in the quiz code the same way.

==> decomposes as == >.
= = > is already decomposed into Rust tokens.
== > is already decomposed.
= => is already decomposed.
Our macro is the same as if we had written the first rule with a space. The third rule is unreachable.

macro_rules! m {
    (== >) => { print!("1"); };
    (= = >) => { print!("2"); };
    (== >) => { print!("3"); };
    (= =>) => { print!("4"); };
}
Within main, the first and third lines both match the first macro rule. The second line matches the second rule and the fourth line matches the fourth rule. The output is 1214.

Procedural macros use a more flexible and powerful macro API and can always distinguish between different spacings of the same characters, such as == > vs ==>.
*/