use machine_factor::{factorize,factorize_128,Factorization128};
/*
   --version
   --help
   --hex  Hexidecimal input
*/
static HELP : &str = "
Usage: factor OPTION x1,x2,..

  Produces prime factors for all the integers provided in the commandline
  If no arguments are provided then integers are read from stdin until q,quit or exit is read.

Options:

  --hex         Reads input as hexidecimal
  
  --version     Returns the version and library about information
  --help        This help page
";

static VERSION : &str = "
factor 1.1
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Copyright (C) 2025 JASory.
";

enum Value{
   Input(u128),
   Error,
   ParseError(String),
   Quit,
}

fn factor(x: u128) -> Factorization128{
   if x < 1u128<<64{
      let t = factorize(x as u64);
      let mut zero = factorize_128(0u128);
      
      for i in 0..t.len{
          zero.powers[i]=t.powers[i];
          zero.factors[i]=t.factors[i] as u128;
      }
      zero.len = t.len;
      return zero;
   }
   factorize_128(x)
}

fn format(x: u128) -> String{
   
   let f = factor(x);
   
   let mut output = String::new();
      if f.powers[0] != 1{
        output+=&(f.factors[0].to_string()+"^"+&f.powers[0].to_string());
      }
      else{
        output+=&(f.factors[0].to_string());
      }
      if f.len > 1{
      for i in 1..f.len{
         if f.powers[i] != 1{
         let pair = f.factors[i].to_string()+"^"+&f.powers[i].to_string();
         output+=&(" * ".to_owned() + &pair);
         }
         else{
         let pair = f.factors[i].to_string();
         output+= &(" * ".to_owned()+&pair);
         }
      }
      }
      output
  }

fn read_integer(hex: bool) -> Value{
   let mut input = String::new();
   
   match std::io::stdin().read_line(&mut input){
     Ok(_) => (),
     Err(_) => return Value::Error,
   }
   
   let k = input.trim();
   
   if k == "q" || k == "quit" || k == "exit"{
      return Value::Quit;
   }
   if hex {
       match u128::from_str_radix(k,16){
         Ok(x) => {return Value::Input(x);},
         Err(_) => {return Value::ParseError(k.to_string());},
       }
   }
   match k.parse::<u128>(){
     Ok(x) => {return Value::Input(x);},
     Err(_) => {return Value::ParseError(k.to_string());}
   }
}

fn repl(hex: bool){
       loop{
          match read_integer(hex){
            Value::Input(x) => println!("{}: {}",x,format(x)), 
            Value::Error => println!("Unable to read from stdin"),
            Value::ParseError(x) => println!("{} is not a valid input",x),
            Value::Quit => break,
          }
       } 
}

fn main() {

      let env_var = std::env::args().collect::<Vec<String>>();
      let args_count = env_var.len()-1;
      if args_count == 1{
       if env_var[1] == "--hex"{
          repl(true);   
        }
        else{
          match env_var[1].parse::<u128>(){
            Ok(x) => println!("{}: {}",x,format(x)),
            Err(_) => {
              match env_var[1].as_str(){
                 "--help" => println!("{}",HELP),
                 "--version" => println!("{}",VERSION),
                 _=> println!("Unrecognised option '{}'",env_var[1]),
              }
             
            },
          }
        }
      }
      // Zero arguments so we just run the repl
      if args_count==0{
        repl(false)
      }
      // This means that 
      if args_count > 1{
        let mut flag = false;
        let mut start_point = 1;
        if env_var[1] == "--hex"{
           flag = true;
           start_point = 2;
        }
        for i in env_var[start_point..].iter(){
           if flag{
              match u128::from_str_radix(i,16){
               Ok(x) => println!("{}: {}",x,format(x)),
               Err(_) => println!("{} is not a valid integer",i),
              }
           }
           else{
             match i.parse::<u128>(){
               Ok(x) => println!("{}: {}",x,format(x)),
               Err(_) => println!("{} is not a valid integer",i),
             }
           }
        }
      }

}
