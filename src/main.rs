use machine_factor::{factorize_128};

static HELP : &str = "
Usage: factor [OPTION,OPTION] x1,x2,..

  Calculates unary number-theoretic functions for 
  all the integers provided in the commandline
  If no arguments are provided then integers are
  read from stdin until q,quit or exit is read.
  All functions have a no-repeat variant callable 
  with {name}-nr, e.g --prime-nr

Options:
  
 Input:
 
  --hex         Reads input as hexadecimal
  --oct         Reads input as octal
  --bin         Reads input as binary
 
 Factorisation:
 
  --no-repeat   Outputs only the factorisation, without repeating input
  --gnu         Outputs GNU factor format
  --gnu-nr      Outputs GNU factor format without repeating input
 
 Number-theory:
 
  --cyclic         Determines if Z/nZ is cyclic
  --cyclic-filter  Returns only if cyclic
  --euler          Euler totient function
  --exp            Least exponent function
  --liouville      Liouville function
  --mobius         Mobius function
  --omega          Counts the number of prime factors of N
  --omega2         Counts the number of prime factors with multiplicity  
  --prime          Outputs true if prime false if composite
  --prime-filter   Returns only if prime
  --sigma          Number of divisors 
  
 Misc:
 
  --about       Returns the version and library about information
  --help        This help page
";

static ABOUT : &str = "

A faster, more versatile Rust port of GNU factor, that is limited to 128-bit.
Supports the classic GNU factor formatting, as well as some other formatting
that may be easier to parse. This library supports hexadecimal inputs, and 
evaluates several other unary number-theoretic functions that rely on 
factorisation, as well as a much faster primality check only. 

License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

version 1.4 

Copyright (C) 2025 JASory.
";

enum Value{
   Input(u128),
   ParseError(String),
   Quit,
}

/*
   Functions to add
   mobius 
   liouville
   iscyclic: check divide by 2 then check for primality
*/

#[derive(Clone,Copy,Debug)]
enum Input{
  Decimal,
  Hex,
  Oct,
  Bin,
}

#[derive(Clone,Copy,Debug,PartialEq)]
enum Style{
  GNU, // GNU factor format  e.g 60: 2 2 3 5
  GNUNoRepeat, // GNU without repeating  2 2 3 5 
  Math, // Correct mathematical representation 60: 2^2 * 3 * 5
  MathNoRepeat, // Returns factorization 2^2 * 3 * 5 
  Prime, // Returns Primality evaluation
  PrimeNoRepeat, // Returns Primality evaluation without repeating
  Sigma,
  SigmaNoRepeat,
  Euler,
  EulerNoRepeat,
  Exp,
  ExpNoRepeat,
  Omega,
  OmegaNoRepeat,
  OmegaTwo,
  OmegaTwoNoRepeat,
  Cyclic,
  CyclicNoRepeat,
  Mobius,
  MobiusNoRepeat,
  Liouville,
  LiouvilleNoRepeat,
  PrimeFilter,
  CyclicFilter,
}

#[derive(Clone,Copy,Debug)]
struct Parameters{
   hexflag: Input,
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

// Sigma function 
fn sigma_eval(x: u128) -> String{
   if x == 0{
      return "Infinite factors".to_string();
   }
   if x == 1{
      return "1".to_string();
   }
   let fctr = factorize_128(x);
   let mut sigma = 1u32;
   for idx in 0..fctr.len{
       sigma*=fctr.powers[idx] as u32+1;
   }
   sigma.to_string()
}


fn euler_eval(x: u128) -> String{
    if x == 1{
       return "1".to_string();
    }
    let mut numerator = 1u128;
    let mut denominator = 1u128;
    
    let fctr = factorize_128(x);
    
    for idx in 0..fctr.len{
      numerator*=fctr.factors[idx]-1;
      denominator*=fctr.factors[idx];
    }
    ((x/denominator)*numerator).to_string()
}


fn lcm(x: u128, y: u128) -> u128{
    let xtwo = x.trailing_zeros();
    let ytwo = y.trailing_zeros();
    let mut min = xtwo;

    if xtwo > ytwo{
       min = ytwo;
    }

    (x/(machine_factor::partial_gcd_128(x>>xtwo,y>>ytwo)<<min))*y
}

fn nth_root(x: u128, n: u8) -> u128{

        let mut est = (x as f64).powf((n as f64).recip()) as u128 + 1;
        let n128 = n as u128;
        let nminus = (n-1) as u128;
        loop {
            let s = est;
            let t = nminus * s + x/s.pow(n as u32 - 1);
            est = t / n128;
            if est >= s {
                return s;
            }
        }
    }
    
fn is_prime_power(x: u128, n: u8) -> bool{
   if n == 1{
     if machine_prime::is_prime_128(x){
        return true;
     }
     else{
       return false;
     }
   }
   let root = nth_root(x,n);
   if root.pow(n as u32) == x && machine_prime::is_prime_128(root){
      return true;
   }
   return false;
}

fn cyclic_eval(mut x: u128) -> String{
   if x == 0{
      return "false".to_string();
   }
   if x < 8{
      return "true".to_string();
   }
   let tzc = x.trailing_zeros();
   
   if tzc==1{
      x>>=1;
   }
   if tzc>1{
      return "false".to_string();
   }
   for i in machine_factor::PRIMES_101{
       let powest = x.ilog2()/i.ilog2();
       if x==(i as u128).pow(powest){
          return "true".to_string();
       }
   }
   for i in 1..15{
      if is_prime_power(x,i){
         return "true".to_string();
      }
   }
   "false".to_string()
}

fn omega_eval(x: u128) -> String{
   if x == 0{
      return "Infinite".to_string();
   }
   if x == 1{
      return "0".to_string();
   }
   factorize_128(x).len.to_string()
}


fn omegatwo_eval(x: u128) -> String{
   if x == 0{
      return "Infinite".to_string();
   }
   if x == 1{
      return "0".to_string();
   }
   let fctr = factorize_128(x);
   let mut res = 0u8;
   for i in 0..fctr.len{
      res+=fctr.powers[i];
    }
   res.to_string()
}

fn mobius_eval(x:u128) -> String{
   if x == 1{
      return "1".to_string();
   }
   let fctr = factorize_128(x);
   
   for i in 0..fctr.len{
     if fctr.powers[i] > 1{
        return "0".to_string();
     }
   }

  if fctr.len&1==0{
    return "1".to_string()
  }

  return "-1".to_string()
}

fn liouville_eval(x: u128) -> String{
   if x == 1{
      return "1".to_string();
   }
   let fctr = factorize_128(x);
   if fctr.len&1==0{
      return "1".to_string();
   }
    "-1".to_string()
}


fn exponent_eval(x: u128) -> String{
       if x == 0{
        return "Infinite".to_string();
       }
       if x == 1{
          return "1".to_string();
       }
       let fctr = factorize_128(x);
       let mut result = 1;
       let mut start = 0;


       if fctr.factors[0]==2{
        let pow2 = fctr.powers[0];
        start=1;
        if pow2 < 3{
           result = 2u128.pow((pow2-1).into());
        }
        else{
          result=2u128.pow((pow2-2).into());
        }
       }
      
       for idx in start..fctr.len{

         let el = fctr.factors[idx];
         let phi =  (el.pow(fctr.powers[idx] as u32)/el)*(el-1);
         result = lcm(result,phi);
       }
    result.to_string()
}
/**/

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
      Style::GNU => x.to_string() + ": " + &gnu_style(x) + "\n",
      Style::GNUNoRepeat => gnu_style(x) + "\n",
      Style::Math => x.to_string() + ": " + &math_style(x) + "\n",
      Style::MathNoRepeat => math_style(x) + "\n",
      Style::Prime => x.to_string() + ": " + &machine_prime::is_prime_128(x).to_string() + "\n",
      Style::PrimeNoRepeat => machine_prime::is_prime_128(x).to_string() + "\n",
      Style::Sigma => x.to_string() + ": " + &sigma_eval(x).to_string() + "\n", 
      Style::SigmaNoRepeat => sigma_eval(x).to_string() + "\n", 
      Style::Euler => x.to_string() + ": " + &euler_eval(x).to_string() + "\n", 
      Style::EulerNoRepeat => euler_eval(x).to_string() + "\n", 
      Style::Exp => x.to_string() + ": " + &exponent_eval(x) + "\n",
      Style::ExpNoRepeat => exponent_eval(x) + "\n",
      Style::Omega => x.to_string()  + ": "  +&omega_eval(x) + "\n",
      Style::OmegaNoRepeat => omega_eval(x) + "\n",
      Style::OmegaTwo => x.to_string() + ": " + &omegatwo_eval(x) + "\n",
      Style::OmegaTwoNoRepeat => omegatwo_eval(x) + "\n",
      Style::Mobius => x.to_string() + ": " +&mobius_eval(x) + "\n",
      Style::MobiusNoRepeat => mobius_eval(x) + "\n",
      Style::Liouville => x.to_string() + ": " + &liouville_eval(x) + "\n",
      Style::LiouvilleNoRepeat => liouville_eval(x) + "\n",
      Style::Cyclic => x.to_string() + ": " + &cyclic_eval(x) + "\n",
      Style::CyclicNoRepeat => cyclic_eval(x) + "\n",
      Style::PrimeFilter => {if machine_prime::is_prime_128(x){return x.to_string()+"\n"}else{String::new()}},
      Style::CyclicFilter => {if cyclic_eval(x)=="true".to_string(){return x.to_string()+"\n"}else{String::new()}},
   }
      
  }
  
fn parse_integer(input: &str, radix: Input) -> Value{
   let k = input.trim();
   
   if k == "q" || k=="quit"||k=="exit"{
      return Value::Quit;
   }

   match radix{
     Input::Decimal => {
        match k.parse::<u128>(){
           Ok(x) => {return Value::Input(x);},
          Err(_) => {return Value::ParseError(k.to_string());},  
        }
     } 
     Input::Hex => {
       match u128::from_str_radix(k,16){
           Ok(x) => {return Value::Input(x);},
          Err(_) => {return Value::ParseError(k.to_string());},
       }
     }
     Input::Oct => {
       match u128::from_str_radix(k,8){
           Ok(x) => {return Value::Input(x);},
          Err(_) => {return Value::ParseError(k.to_string());},
       }
     }
     Input::Bin => {
       match u128::from_str_radix(k,2){
           Ok(x) => {return Value::Input(x);},
          Err(_) => {return Value::ParseError(k.to_string());},
      }
     }
   }
}

fn param_set(env: &Vec<String>) -> Parameters{
   let mut def_hex = Input::Decimal;
   let mut def_style = Style::Math;
   let mut sidx : usize = 0;
   
  for (eidx,el) in env[1..].iter().enumerate(){
  #[allow(unused_comparisons)] // compiler erroneously flags this with a warning 
    if eidx >= 0 && eidx < 4{
    
    match el.as_str(){
      "--hex" => {def_hex = Input::Hex;}
      "--oct" => {def_hex = Input::Oct;}
      "--bin" => {def_hex= Input::Bin;}
      "--gnu" => {def_style = Style::GNU;}
      "--gnu-nr" => {def_style = Style::GNUNoRepeat;}
      "--no-repeat" => {def_style = Style::MathNoRepeat;}
      "--prime" => {def_style = Style::Prime;}
      "--prime-nr" => {def_style=Style::PrimeNoRepeat;} 
      "--sigma" => {def_style = Style::Sigma;}
      "--sigma-nr" => {def_style=Style::SigmaNoRepeat;}
      "--euler" => {def_style = Style::Euler;}
      "--euler-nr" => {def_style=Style::EulerNoRepeat;}
      "--exp" => {def_style=Style::Exp;}
      "--exp-nr" => {def_style=Style::ExpNoRepeat;}
      "--omega" => {def_style=Style::Omega;}
      "--omega-nr" => {def_style=Style::OmegaNoRepeat;}
      "--omega2" => {def_style=Style::OmegaTwo;}
      "--omega2-nr" => {def_style=Style::OmegaTwoNoRepeat;}
      "--mobius" => {def_style=Style::Mobius;}
      "--mobius-nr" => {def_style=Style::MobiusNoRepeat;}
      "--liouville" => {def_style=Style::Liouville;}
      "--liouville-nr" => {def_style=Style::LiouvilleNoRepeat;}
      "--cyclic" => {def_style=Style::Cyclic;}
      "--cyclic-nr" => {def_style=Style::CyclicNoRepeat;}
      "--prime-filter" => {def_style=Style::PrimeFilter;}
      "--cyclic-filter" => {def_style=Style::CyclicFilter;}
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
                Value::Input(num) => print!("{}",format(num,params.format)),
                Value::ParseError(error_mess) => {println!("'{}' is not a valid positive integer",error_mess);},
                Value::Quit => {break;},
              }
          }
         Err(error_mess) => {println!("{}",error_mess)} 
        }
   }
}

fn greedy(params: Parameters,env_var: &Vec<String>){
        for i in env_var[params.idx+1..].iter(){
           match params.hexflag{

               Input::Decimal =>{

              match i.parse::<u128>(){
               Ok(x) => print!("{}",format(x,params.format)),
               Err(_) => println!("'{}' is not a valid positive integer",i),
              }
               }
               Input::Hex =>{

              match u128::from_str_radix(i,16){
               Ok(x) => print!("{}",format(x,params.format)),
               Err(_) => println!("'{}' is not a valid positive integer",i),
              }
               }
               Input::Oct =>{

              match u128::from_str_radix(i,8){
               Ok(x) => print!("{}",format(x,params.format)),
               Err(_) => println!("'{}' is not a valid positive integer",i),
              }
               }
               Input::Bin =>{
                 match u128::from_str_radix(i,2){
                    Ok(x) => print!("{}",format(x,params.format)),
                    Err(_) => println!("'{}' is not a valid positive integer",i),
                 }
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
                    "--h" => println!("{}",HELP),
                    "--about" => println!("{}",ABOUT),
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
