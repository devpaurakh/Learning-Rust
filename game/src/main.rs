use std::{collections::HashMap, fmt, io};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
#[derive(Debug,FromPrimitive,Eq,PartialEq,Hash)]
enum Gem {
    Diamond = 1,
    Sapphire,
    Ruby,
    Topaz,
    Onyx,
    Jade,
}

impl fmt::Display for Gem  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gem::Diamond => write!(f, "diamond"),
            Gem::Sapphire => write!(f, "diamond"),
            Gem::Ruby => write!(f, "diamond"),
            Gem::Topaz => write!(f, "diamond"),
            Gem::Onyx => write!(f, "diamond"),
            Gem::Jade => write!(f, "diamond"), 
        }
    }
    
}

fn gameFunction(map: &mut [[u8;5];5]) -> Vec<Gem>{

    let mut foundGem: Vec<Gem> = Vec::new();

    while foundGem.len() < 5 {
        
   


     println!("Search and XY position 0 to 4 (Example input: 4 2)");
     let mut input = String::new();

      match io::stdin().read_line(&mut input){
        Ok(_)=>{}
        Err(_)=>{
            println!("Failed to read line");
        }
      };

     let parts: Vec<&str> = input.trim().split_whitespace().collect();

     if parts.len() != 2{
        println!("Two number required");
        continue;
     }

     let (x,y) = match (parts[0].parse::<u8>(),parts[1].parse::<u8>()) {

        (Ok(x),Ok(y)) => (x,y),
         
         _ => {
            println!("Something wrong with the program");
            continue;
         }
     };

     if x >= 5 || y >=5 {

        println!("Invlaid Index or position")

     }

    let data = map[x as usize][y as usize];

    let gem = match Gem::from_u8(data) {

        Some(gem) => gem,
        None =>{
            println!("No gem not found at this position");
            continue;
        },
        
    };
    foundGem.push(gem);
    map[x as usize][y as usize] = 0;
    println!("{foundGem:?}")
    } 

    return  foundGem;

}
fn main() {
    let mut map = [[0;5];5];
    // println!("{map:?}");


    map[4][2] = 1;
    map[4][1] = 5;
    map[3][4] = 6;
    map[1][2] = 2;
    map[4][2] = 6;
    map[2][3] = 1;

    // println!("{map:?}");

    for row in map{
        println!("{row:?}")
    }
    let mut foundGem:Vec<Gem> = gameFunction(&mut map);

    
    println!("You have found all the gems");

    let mut gem_values:HashMap<Gem, f64> = HashMap::new();

    gem_values.insert(Gem::Diamond, 400.00);
    gem_values.insert(Gem::Jade, 400.00);
    gem_values.insert(Gem::Onyx, 10.00);
    gem_values.insert(Gem::Ruby, 100.00);
    gem_values.insert(Gem::Sapphire, 1.00);
    gem_values.insert(Gem::Topaz, 99.00);

    let mut  sum = 0.0;
    for gem in foundGem{
        sum += gem_values[&gem];  
    }

    println!("Print total worth of the gem is {}",sum );

}
