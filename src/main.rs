
use std::io;
use rand::Rng;

struct Player {
    player_no: u8,
    name: String,
    current_score: u8,
    total_score: u8
}
 
impl Player{
    fn new(playername:String,player_no:u8) -> Player{
        Player {
            player_no : player_no,
            name: playername,
            current_score: 0,
            total_score: 0
        }
    }
}

    fn dice_roll() -> u8{
        let dice_num:u8 = rand::thread_rng().gen_range(1, 19);
        match dice_num{
            18 => 0,
            _ => dice_num,
        }  
            
        
    }



fn main() {
let mut isWin = false;
let mut dice:Vec<Player> = Vec::new();
let mut temp = String::new();

println!("Enter Number of Players");
io::stdin().read_line(&mut temp).expect("Number Only");

let no_of_players: u8;
no_of_players = temp.trim().parse::<u8>().unwrap();
let mut player_object:Player;
for x in 1..=no_of_players {
    let mut temp2 = String::new();
    println!("Enter Player's {:?} Name: ?",x);
    let player_name: String;
    io::stdin().read_line(&mut temp2).expect("String Only");
    player_name = temp2.trim().parse::<String>().unwrap();

    player_object = Player::new(player_name,x);
    dice.push(player_object);
}
println!("Let's Start Play: Hit Enter to Continue Play");
println!("Press any other key to Quit Play");
println!("The Team For This Game");

// println!("1st Random: {:?}",rand::thread_rng().gen_range(1, 101));
// println!("2nd Random: {:?}",rand::thread_rng().gen_range(1, 101));
// println!("3rd Random: {:?}",rand::thread_rng().gen_range(1, 101));
for p1 in 0..=dice.len()-1
{
    println!("Player # {} - {}",dice[p1].player_no,dice[p1].name);
}
let mut turn_no:u8 = 1;
while true{  
    let mut inc = String::new();  
    io::stdin().read_line(&mut inc).expect("failed to read key");
   
    for j in 0..=dice.len()-1
    {
        let draw = dice_roll();
        dice[j].current_score = draw;
        if dice[j].total_score + dice[j].current_score <= 99
        {
            
           let beat_it_score = dice[j].total_score + dice[j].current_score;
           for find in 0..=dice.len()-1
           {
               if dice[find].total_score == beat_it_score && find != j
                {
                    dice[find].total_score = 0; 
                    println!("Alas! {} has kicked by {} at score of {}",dice[find].name,dice[j].name,beat_it_score);
                    break;
                }
           }
           dice[j].total_score = beat_it_score;
        }
        if dice[j].total_score + dice[j].current_score == 100
           {
                dice[j].total_score = 100;
                println!("Congratulations! Player {} has won on turn {}",dice[j].name,turn_no);
                isWin = true;

                break;
            }
    }
    for print in 0..=dice.len()-1{
        if isWin
           { break; }
    println!("Turn: {} Dice Roll of Player {} - {} is {} and total {}",turn_no,dice[print].player_no,dice[print].name,dice[print].current_score,dice[print].total_score);
        
    }
    turn_no = turn_no + 1; 
   
     
    if inc != "\r\n" || isWin
        

           { break; }
        
}

}

 


