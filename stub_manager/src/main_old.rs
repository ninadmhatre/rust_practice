/*
read command line argument
check which numbers are
   - if divisible by 3 print fiz
   - if by 5 - buzz
   - if by 3 & 5 - fizbuzz
*/

fn read_input() -> i32 {
    let x = 16;

    x
}

fn main() {
    for n in {1 .. read_input()} {
        if (n % 3 == 0) && (n % 5 == 0) {
            println!("{:?} - fizbuzz", n);
        } else if n % 5 == 0 {
            println!("{:?} - buzz", n);
        } else if n % 3 == 0 {
            println!("{:?} - fiz", n);
        } else {
            println!("{:?}", n);
        }
    }
}
