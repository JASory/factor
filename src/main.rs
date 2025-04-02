use machine_factor::{factorize_128};

static HELP : &str = "
Usage: factor [OPTION,OPTION] x1,x2,..

  Produces prime factors for all the integers provided in the commandline
  If no arguments are provided then integers are read from stdin until q,quit or exit is read.

Options:

  --hex         Reads input as hexadecimal
  --no-repeat   Outputs only the factorisation, without repeating input
  --gnu         Outputs GNU factor format
  --gnu-nr      Outputs GNU factor format without repeating input
  
  --version     Returns the version and library about information
  --help        This help page
";

static VERSION : &str = "
factor 1.3
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

#[derive(Clone,Copy,Debug)]
enum Style{
  GNU, // GNU factor format  e.g 60: 2 2 3 5
  GNUNoRepeat, // GNU without repeating  2 2 3 5 
  Math, // Correct mathematical representation 60: 2^2 * 3 * 5
  MathNoRepeat, // Returns factorization 2^2 * 3 * 5 
}

#[derive(Clone,Copy,Debug)]
struct Parameters{
   hexflag: bool,
   format: Style,
   idx: usize, 
}

// Return Infinite for 0 and 1 for 1
fn math_style(x: u128) -> String{
   if x == 0{
       return "All integers".to_string();
   }
   
   if x == 1{
      return "1".to_string();
   }
   
   let f = factorize_128(x);
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

fn gnu_style(x: u128) -> String{
       if x < 2{
         return "".to_string();
       }
   
       let f = factorize_128(x);
       let mut output = String::new();
       for idx in 0..f.len{
          for _ in 0..f.powers[idx]{
             output+=&(f.factors[idx].to_string()+" ");
          }
       }
       output
}

fn format(x: u128, style: Style) -> String{
   
   match style {
      Style::GNU => x.to_string() + ": " + &gnu_style(x),
      Style::GNUNoRepeat => gnu_style(x),
      Style::Math => x.to_string() + ": " + &math_style(x),
      Style::MathNoRepeat => math_style(x),
   }
      
  }

fn parse_integer(input: &str, hex: bool) -> Value{
   let k = input.trim();
   
   if k == "q" || k=="quit"||k=="exit"{
      return Value::Quit;
   }

   if hex {
      match u128::from_str_radix(k,16){
        Ok(x) => {return Value::Input(x);},
       Err(_) => {return Value::ParseError(k.to_string());},
      }
   }
   else{
     match k.parse::<u128>(){
        Ok(x) => {return Value::Input(x);},
        Err(_) => {return Value::ParseError(k.to_string());},
     }
   }
}

fn param_set(env: &Vec<String>) -> Parameters{
   let mut def_hex = false;
   let mut def_style = Style::Math;
   let mut sidx : usize = 0;
   
  for (eidx,el) in env[1..].iter().enumerate(){
  #[allow(unused_comparisons)] // compiler erroneously flags this with a warning 
    if eidx >= 0 && eidx < 4{
    
    match el.as_str(){
      "--hex" => {def_hex = true;}
      "--gnu" => {def_style = Style::GNU;}
      "--gnu-nr" => {def_style = Style::GNUNoRepeat;}
      "--no-repeat" => {def_style = Style::MathNoRepeat;}
      _=> {sidx=eidx;break;},
    }
    
    }
    
    }
  
   Parameters{hexflag : def_hex, format : def_style,idx: sidx}
}


fn pipe(params: Parameters){
   use std::io::{stdin,BufRead};
   for line in stdin().lock().lines(){
       match line{
          Ok(l) => {
              match parse_integer(&l,params.hexflag){
                Value::Input(num) =>{println!("{}",format(num,params.format))},
                Value::ParseError(error_mess) => {println!("{}",error_mess);},
                Value::Error => {println!("Unknown error");},
                Value::Quit => {break;},
              }
          }
         Err(error_mess) => {println!("{}",error_mess)} 
        }
   }
}

fn greedy(params: Parameters,env_var: &Vec<String>){
        for i in env_var[params.idx+1..].iter(){
           if params.hexflag{
              match u128::from_str_radix(i,16){
               Ok(x) => println!("{}",format(x,params.format)),
               Err(_) => println!("'{}' is not a valid integer",i),
              }
           }
           else{
             match i.parse::<u128>(){
               Ok(x) => println!("{}",format(x,params.format)),
               Err(_) => println!("'{}' is not a valid integer",i),
             }
           }
        }
}

fn main() {

      let env_var = std::env::args().collect::<Vec<String>>();
      let args_count = env_var.len()-1;
      let params = param_set(&env_var);
     
      if params.idx==0{
         if args_count == 0{
            pipe(params);
         }
         else{
         match env_var[1].parse::<u128>(){
            Ok(_)=> greedy(params,&env_var),
            Err(_)=> {
                match env_var[1].as_str(){
                    "--help" => println!("{}",HELP),
                    "--version" => println!("{}",VERSION),
                    _=> pipe(params),
                }
            },
         }
        } 
      }
      
      if params.idx != 0{
        greedy(params,&env_var);
      }
}
