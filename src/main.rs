
mod message;
mod functions;
mod evaluator;
mod io;
mod math;

use message::{HELP,ABOUT};
use crate::math::math_style;

use io::{greedy,pipe,param_set,ParamParse};

fn runner(){
      let env_var = std::env::args().collect::<Vec<String>>();
      let args_count = env_var.len()-1;
      let p = param_set(&env_var);

      if args_count == 0{

        match p{
          ParamParse::Param(params) => pipe(params),
          _=> eprintln!("Error input"),
        }
         
      }
      if args_count == 1{
         match env_var[1].as_str(){
           "--help" => println!("{}",HELP),
           "--h" => println!("{}",HELP),
           "--about" => println!("{}",ABOUT),
           _=> { match env_var[1].parse::<u128>(){
                  Ok(x) => println!("{} : {}",x,math_style(x)),
                  Err(_) =>  match p{
                         ParamParse::Param(params) => pipe(params),
                         _=> eprintln!("Error input"),
                         },
                 }
                },
         } 
      }
      if args_count > 1{

      match p{
        ParamParse::Param(params) => {
           if args_count == params.idx{
              pipe(params)
           } else {
             greedy(params,&env_var)
           }
        }  
        ParamParse::Undefined => eprintln!("Undefined inputs"),
        ParamParse::FuncConflict => eprintln!("Conflicting functions called"),
        ParamParse::RadixConflict => eprintln!("Conflicting Radix called"),
        ParamParse::MissingArg => eprintln!("Missing Fixed value argument"),
      }
      }
}

fn main() {
      runner()
}
