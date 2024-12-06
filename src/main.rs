use std::{thread, time, io};

fn main() {
    let timetime = time::Duration::from_millis(125);
    let mut counter = 1;
    let mut iter = 0;
    
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
