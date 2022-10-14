extern crate rand;
use rand::prelude::*;
use text_io::try_read;


struct Player {
    number: i32
} 


fn main(){
    let mut rng = thread_rng();
    let boxes: [i32; 20] = [1 ,100, 1000, 15000, 75000, 5, 250, 3000, 20000, 100,000, 10, 500, 5000, 35000, 250000, 50, 750, 10000, 50000 ];

    let box_1:i32 = boxes[rng.gen_range(0, 20)];
    let box_2:i32 = boxes[rng.gen_range(0, 20)];
    let box_3:i32 = boxes[rng.gen_range(0, 20)];
    let box_4:i32 = boxes[rng.gen_range(0, 20)];
    let box_5:i32 = boxes[rng.gen_range(0, 20)];

    println!("welcome to deal or no deal");
    println!("Begin:");
    loop {
        println!("there a 5 boxes infront of you");
        println!("choose a box 1 - 5");
        let number:Result<i32, _> = try_read!("{}");
       let chosen_box1 = match number {
            Ok(number) => Player{number},
            Err(_) => continue
          }; 
        let result_1 = chosen_box1.number;
        let outcome_1:i32 = box_2 + box_3 + box_4 + box_5;
        let outcome_2:i32 = box_1 + box_3 + box_4 + box_5;
        let outcome_3:i32 = box_1 + box_2 + box_4 + box_5;
        let outcome_4:i32 = box_1 + box_2 + box_3 + box_5;
        let outcome_5:i32 = box_1 + box_2 + box_3 + box_4;
        
        if result_1 == 1 || result_1 == 2 || result_1 == 3   || result_1 == 4  || result_1 == 5 {
            println!("you have chosen box number {}", result_1);
            if result_1 == 1{
                println!("you have lost out on £{}", box_1);
                println!("but you have won £{}", outcome_1)
                
            }
            else if result_1 == 2 {
                println!("you have lost out on £{}", box_2);
                println!("but you have won £{}", outcome_2)
                
            }
            else if result_1 == 3 {
                println!("you have lost out on £{}", box_3);
                println!("but you have won £{}", outcome_3)
                
            }
            else if result_1 == 4 {
                println!("you have lost out on £{}", box_4);
                println!("but you have won £{}", outcome_4)
                
            }
            else if result_1 == 5 {
                println!("you have lost out on £{}", box_5);
                println!("but you have won £{}", outcome_5)
                
            }   
            break;
        } else {
            println!("please input a number");
            continue;
        }  
    } 

}
