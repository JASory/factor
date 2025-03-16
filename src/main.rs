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
factor 1.2
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

fn repl(hex: bool, sty: Style){
       loop{
          match read_integer(hex){
            Value::Input(x) => println!("{}",format(x,sty)), 
            Value::Error => println!("Unable to read from stdin"),
            Value::ParseError(x) => println!("{} is not a valid input",x),
            Value::Quit => break,
          }
       } 
}


fn param_set(env: &Vec<String>) -> (bool,Style,usize){
   let mut def_hex = false;
   let mut def_style = Style::Math;
   let mut idx : usize = 0;
   
  for (eidx,el) in env[1..].iter().enumerate(){
    if eidx > 0 && eidx < 4{
    
    match el.as_str(){
      "--hex" => {def_hex = true;}
      "--gnu" => {def_style = Style::GNU;}
      "--gnu-nr" => {def_style = Style::GNUNoRepeat;}
      "--no-repeat" => {def_style = Style::MathNoRepeat;}
      _=> {idx=eidx;break;},
    }
    
    }
    
    }
  
   (def_hex,def_style,idx)
}

fn greedy(params: (bool,Style,usize),env_var: &Vec<String>){
        for i in env_var[params.2+1..].iter(){
           if params.0{
              match u128::from_str_radix(i,16){
               Ok(x) => println!("{}",format(x,params.1)),
               Err(_) => println!("'{}' is not a valid integer",i),
              }
           }
           else{
             match i.parse::<u128>(){
               Ok(x) => println!("{}",format(x,params.1)),
               Err(_) => println!("'{}' is not a valid integer",i),
             }
           }
        }
}

fn main() {

      let env_var = std::env::args().collect::<Vec<String>>();
      let args_count = env_var.len()-1;
      let params = param_set(&env_var);
      
      
      if params.2==0{
         if args_count == 0{
            repl(params.0,params.1);
         }
         else{
         match env_var[1].parse::<u128>(){
            Ok(_)=> greedy(params,&env_var),
            Err(_)=> {
                match env_var[1].as_str(){
                    "--help" => println!("{}",HELP),
                    "--version" => println!("{}",VERSION),
                    _=> repl(params.0,params.1),
                }
            },
         }
        } 
      }
      
      if params.2 != 0{
        greedy(params,&env_var);
      }
}
