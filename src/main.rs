use rand::Rng;
use std:: fs;
use std::collections::HashMap;


struct Config{//data from config file 
   pull_day:u32,
   pull_max:u32,
   push_day:u32,
   push_max:u32,
}

  
fn main() {

   const PATH: &str  = "src/config.ttt"; //config path 

   let config = Config{  //get data from config file in config variable
      pull_day:  match read_config(PATH).get("pull_day") {
               Some(value) => *value,
               None => 0,
            } ,
      pull_max:  match read_config(PATH).get("pull_max") {
               Some(value) => *value,
               None => 0,
            }, 
      push_day:  match read_config(PATH).get("push_day") {
               Some(value) => *value,
               None => 0,
            } ,
      push_max:  match read_config(PATH).get("push_max") {
               Some(value) => *value,
               None => 0,
            } 
   };
  
  config.pull_ups();
  config.push_ups();
}


//function for read a config file
fn read_config(path: &str) -> HashMap<String, u32>{
   let mut config:  HashMap<String, u32>  = HashMap::new();
   let file = fs::read_to_string(path).expect("File can't be read");
   for line in file.lines() {
      let part: Vec<&str> = line.split(":").collect();
      if part.len() == 2{
         config.insert(part[0].to_string(),part[1].parse().expect("Not work"));
      }
   }
   return config;
}


//function for calcul how many workout need to do
fn training(day:u32,max:u32) -> [i32; 5] {
      let rep = rand::thread_rng().gen_range(2..=4);  
      let mut workout:[i32 ; 5] = [0,0,0,0,0];
  
   
      let procent = if day == 1 {60}
                  else if day == 2 {70}
                  else if day == 3 {80}
                  else {0};
                  



      if max == 0 || max == 1{
         for x in 0..=4{
            workout[x] = 1;
         }
      }
      else {
         for x  in 0..=4 {
            let  y =(max as i32 * procent ) / 100 - x as i32 ;
            if y <= 0{
               workout[x] = 1;
            }
            else { workout[x] = y }
         }
      }
      workout[rep] = workout[rep-1];
      return workout;
   }



impl Config {
   
   fn pull_ups(&self){
      let pull_ups = vec!["regular","close","wind","alternate","close alternate","side to side"];//random get a type of pull ups
      println!("Need to do {} pull ups of {:?}",pull_ups[rand::thread_rng().gen_range(0..=pull_ups.len()-1)],training(self.pull_day,self.pull_max));
   }

   fn push_ups(&self){
      let push_ups = vec!["regular","close","wide","one leg","diamond","decline","incline"];//random get a type of push ups
      println!("Need to do {} push ups of {:?}",push_ups[rand::thread_rng().gen_range(0..=push_ups.len()-1)],training(self.push_day,self.push_max));
   }


}



