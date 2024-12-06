use std::{thread, time, io};

fn main() {
    let timetime = time::Duration::from_millis(125);
    let mut counter = 1;
    let mut iter = 0;

    /*
     2D |   | COL                                     COL
    ____|___|__1__2__3__4__5__6__7__8__9__10__11__12__13___
    ROW | 1 |  J  J
        | 2 |  U  U 
        | 3 |  L  L 
        | 4 |  I  I
        | 5 |  A  A
        | 6 |  N  N
        | 7 |  E  E
        | 8 |  K  K
        | 9 |  K  K
        |10 |
        |11 |  M  M
        |12 |  A  A
        |13 |  T  T
        |14 |  R  R
        |15 |  I  I
    ROW |16 |  X  X
    */
    
    let julianekk =
    [" J", " U", " L", " I", " A", " N", " E", " K", " K", " ", 
    " M", " A", " T", " R", " I", " X"];

    while counter <= 16 {
        let letter = julianekk[iter];
        println!("{}", letter);
        thread::sleep(timetime);
        iter += 1;
        counter += 1;
    }
    
    println!("\nPRESS ENTER TO EXIT PROGRAM");
    let _ = io::stdin().read_line(&mut String::new());
}