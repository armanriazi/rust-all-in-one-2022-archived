

fn main()  {
    let r;//This code won’t compile because the value r is referring to has gone out of scope before we try to use it

    {
        let x = 5;
        r = &x;//The variable x doesn’t “live long enough.” The reason is that x will be out of scope when the inner scope ends on line 7
    }

    println!("r: {}", r);
}
/* dangle
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
*/

/* undangle
fn main() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}

*/