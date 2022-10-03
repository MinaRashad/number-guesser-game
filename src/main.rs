use std::io::{self, Write};

fn main() {
    println!("Hello, I can guess any number you choose between 1 and 1,000,000\n IN 20 TRIES ONLY!!!!!!");
    println!("Now choose a number in your head when ready press 'Enter'");

    let mut entered = String::new();
    let mut try_num = 0;
    let mut left = 0;
    let mut right = 1_000_000;
    let mut win = false;
    io::stdin().read_line(&mut entered)
        .expect("You entered some exit code");


    loop {
        entered = String::from("");
        println!("Try#{} : I guess it is {}\n",try_num,(right+left)/2);
        
        let mut choose = false;
        
        while !choose {
            print!("write (higher, lower, correct) > ");
            io::stdout().flush().expect("Err Flushing");
            io::stdin().read_line(&mut entered)
                .expect("Err: You entered an exit code");

            match entered.trim() {
                "higher" => {left = (right+left)/2; try_num +=1; choose=true },
                "lower" => {right = (right+left)/2; try_num +=1; choose=true },
                "correct" => {win = true },
                _=> println!("Try again"),
            }
        }
        if try_num >=20  {
            println!("You won :(");
            break;
        }

        if win  {
            println!("I won :D");
            break;
        }
        


    }
}
