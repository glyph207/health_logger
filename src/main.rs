use std::io;

fn main() {
    println!("welcome to health logger!");
    println!("type 'list' for list of commands, 'exit' or, well, command");
    let mut user_input = String::new();

    loop {
        user_input.clear();
        //println!("user input is {}", user_input);
        io::stdin().read_line(&mut user_input)
            .expect("failed to read line");

        match user_input.trim().as_ref() {
            "list" => {
                println!("* list");
                println!("* exit");
                println!("other commands are in development, sorry");
            },
            "exit" => {
                println!("thanx for using health logger, bye!");
                break;
            },
            _ => println!("unrecognized command, plz try again"),
        };
    }
}
