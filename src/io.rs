use crate::functions::{Unary,Binary,Function};
use crate::evaluator::function_eval;

#[derive(Clone,Copy,Debug)]
enum Radix{
  Decimal=10,
  Hex=16,
  Oct=8,
  Bin=2,
}

enum Value{
   Input(u128),
   ParseError(String),
   Quit,
}

#[derive(Clone,Copy)]
pub(crate) struct Parameters{
              style: Radix,
              func: Function,
   pub(crate) fixed: Option<u128>,
   pub(crate) idx: usize,
   pub(crate) binary: bool,
}

impl Parameters{
   fn new(style: Radix, func: Function, fixed: Option<u128>,idx: usize, binary: bool) -> Self{
      Self{style: style,func: func, fixed: fixed, idx: idx, binary: binary}
   }
}

#[derive(Clone,Copy)]
pub(crate) enum ParamParse{
   FuncConflict,
   RadixConflict,
   Undefined,
   MissingArg,
   Param(Parameters),
}



fn radix_parse(x: &str) -> Option<Radix>{
   match x{
     "--hex" => Some(Radix::Hex),
     "--oct" => Some(Radix::Oct),
     "--bin" => Some(Radix::Bin),
     _=> None,
  }

}

fn unary_parse(x: &str) -> Option<Function>{
    match x {
      "--gnu" => Some(Function::ArityOne(Unary::GNU)),
      "--gnu-nr" => Some(Function::ArityOne(Unary::GNUNoRepeat)),
      "--no-repeat" => Some(Function::ArityOne(Unary::FactorNoRepeat)),
      "--max" => Some(Function::ArityOne(Unary::MaxFactor)),
      "--max-nr" => Some(Function::ArityOne(Unary::MaxFactorNoRepeat)),
      "--prime" => Some(Function::ArityOne(Unary::Prime)),
      "--composite" => Some(Function::ArityOne(Unary::Composite)),
      "--prime-nr" => Some(Function::ArityOne(Unary::PrimeNoRepeat)),
      "--composite-nr" => Some(Function::ArityOne(Unary::CompositeNoRepeat)),
      "--sigma" => Some(Function::ArityOne(Unary::Sigma)),
      "--sigma-nr" => Some(Function::ArityOne(Unary::SigmaNoRepeat)),
      "--euler" => Some(Function::ArityOne(Unary::Euler)),
      "--euler-nr" => Some(Function::ArityOne(Unary::EulerNoRepeat)),
      "--exp" => Some(Function::ArityOne(Unary::Exp)),
      "--exp-nr" => Some(Function::ArityOne(Unary::ExpNoRepeat)),
      "--omega" => Some(Function::ArityOne(Unary::Omega)),
      "--omega-nr" => Some(Function::ArityOne(Unary::OmegaNoRepeat)),
      "--omega-m" => Some(Function::ArityOne(Unary::OmegaTwo)),
      "--omega-m-nr" => Some(Function::ArityOne(Unary::OmegaTwoNoRepeat)),
      "--mobius" => Some(Function::ArityOne(Unary::Mobius)),
      "--mobius-nr" => Some(Function::ArityOne(Unary::MobiusNoRepeat)),
      "--liar" => Some(Function::ArityOne(Unary::Liar)),
      "--liar-nr" => Some(Function::ArityOne(Unary::LiarNoRepeat)),
      "--strong-liar" => Some(Function::ArityOne(Unary::StrongLiar)),
      "--strong-liar-nr" => Some(Function::ArityOne(Unary::StrongLiarNoRepeat)),
      "--liouville" => Some(Function::ArityOne(Unary::Liouville)),
      "--liouville-nr" => Some(Function::ArityOne(Unary::LiouvilleNoRepeat)),
      "--cyclic" => Some(Function::ArityOne(Unary::Cyclic)),
      "--cyclic-nr" => Some(Function::ArityOne(Unary::CyclicNoRepeat)),
      "--prime-filter" => Some(Function::ArityOne(Unary::PrimeFilter)),
      "--composite-filter" => Some(Function::ArityOne(Unary::CompositeFilter)),
      "--cyclic-filter" => Some(Function::ArityOne(Unary::CyclicFilter)),
      "--unit-ratio" => Some(Function::ArityOne(Unary::UnitRatio)),
      "--unit-ratio-nr" => Some(Function::ArityOne(Unary::UnitRatioNoRepeat)),
      "--unit-ratio-d" => Some(Function::ArityOne(Unary::UnitRatiod)),
      "--unit-ratio-d-nr" => Some(Function::ArityOne(Unary::UnitRatiodNoRepeat)),
      "--frobenius-idx" => Some(Function::ArityOne(Unary::FrobeniusIdx)),
      "--frobenius-idx-nr" => Some(Function::ArityOne(Unary::FrobeniusIdxNoRepeat)),
      _=> None,
     }
    }
    
   fn binary_parse(x: &str) -> Option<Function>{
     match x{
      "--residue" => Some(Function::ArityTwo(Binary::Residue)),
      "--residue-nr" => Some(Function::ArityTwo(Binary::ResidueNoRepeat)),
      "--residue-swap" => Some(Function::ArityTwo(Binary::ResidueSwap)),
      "--residue-swap-nr" => Some(Function::ArityTwo(Binary::ResidueSwapNoRepeat)),
      "--gcd" => Some(Function::ArityTwo(Binary::Gcd)),
      "--gcd-nr" => Some(Function::ArityTwo(Binary::GcdNoRepeat)),
      "--kronecker" => Some(Function::ArityTwo(Binary::Kronecker)),
      "--kronecker-nr" => Some(Function::ArityTwo(Binary::KroneckerNoRepeat)),
      "--kronecker-swap" => Some(Function::ArityTwo(Binary::KroneckerSwap)),
      "--kronecker-swap-nr" => Some(Function::ArityTwo(Binary::KroneckerSwapNoRepeat)),
      "--lcm" => Some(Function::ArityTwo(Binary::Lcm)),
      "--lcm-nr" => Some(Function::ArityTwo(Binary::LcmNoRepeat)),
      "--coprime" => Some(Function::ArityTwo(Binary::Coprime)),
      "--coprime-nr" => Some(Function::ArityTwo(Binary::CoprimeNoRepeat)),
      "--coprime-filter" => Some(Function::ArityTwo(Binary::CoprimeFilter)),
      "--fermat-filter" => Some(Function::ArityTwo(Binary::FermatFilter)),
      "--strong-filter" => Some(Function::ArityTwo(Binary::StrongFermatFilter)),
      "--fermat" => Some(Function::ArityTwo(Binary::Fermat)),
      "--fermat-swap" => Some(Function::ArityTwo(Binary::FermatSwap)),
      "--strong" => Some(Function::ArityTwo(Binary::StrongFermat)),
      "--strong-swap" => Some(Function::ArityTwo(Binary::StrongFermatSwap)),
      "--fermat-nr" => Some(Function::ArityTwo(Binary::FermatNoRepeat)),
      "--fermat-swap-nr" => Some(Function::ArityTwo(Binary::FermatSwapNoRepeat)),
      "--strong-nr" => Some(Function::ArityTwo(Binary::StrongFermatNoRepeat)),
      "--strong-swap-nr" => Some(Function::ArityTwo(Binary::StrongFermatSwapNoRepeat)),
      "--order" => Some(Function::ArityTwo(Binary::Ord)),
      "--order-nr" => Some(Function::ArityTwo(Binary::OrdNoRepeat)),  
      "--order-swap" => Some(Function::ArityTwo(Binary::OrdSwap)),
      "--order-swap-nr" => Some(Function::ArityTwo(Binary::OrdSwapNoRepeat)),      
      "--inverse" => Some(Function::ArityTwo(Binary::MulInverse)),
      "--inverse-nr" => Some(Function::ArityTwo(Binary::MulInverseNoRepeat)),
      "--inverse-swap-nr" => Some(Function::ArityTwo(Binary::MulInverseSwapNoRepeat)),
      "--inverse-swap" => Some(Function::ArityTwo(Binary::MulInverseSwap)),
      _=> None,
      }
}


// Input parser

 // Find the index of the first integer up to 
 
 pub(crate) fn param_set(env: &Vec<String>) -> ParamParse{
   
   let mut def_style = Radix::Decimal;
   let mut styleflag = false;
   let mut def_func = Function::ArityOne(Unary::Factor);
   let mut funcflag = false;
   let mut sidx : usize = 0;
   let mut binary : bool = false;
   let mut fxd : Option<u128> = Some(0);
   

   for (eidx,el) in env[1..].iter().enumerate(){
      if eidx > 3{
       // halt if greater than 3 args exist
         break;
      }

      let mut eval_flag = false;
      // Attempt to parse argument as a radix parameter
      match radix_parse(el){
        Some(x) => {
           if !styleflag{
              def_style=x;
              styleflag=true;
              eval_flag = true;
              sidx+=1;
              } else{
                return ParamParse::RadixConflict;
              }}
        None => (),
      }
      
     if !eval_flag { 
     match unary_parse(el){
        Some(x) => {
           if !funcflag{
              def_func=x;
              funcflag=true;
              eval_flag=true;
              sidx+=1;
              } else{
                return ParamParse::FuncConflict;
              }}
        None => (),
      }
     }
     if !eval_flag{ 
     match binary_parse(el){
        Some(x) => {
           if !funcflag{
              def_func=x;
              funcflag=true;
              eval_flag=true;
              binary = true;
              sidx+=1;
              } else{
                return ParamParse::FuncConflict;
              }}
        None => (),
      }
     }
     if !eval_flag{
      // Attempt to 
      match parse_integer(el,def_style){
       Value::Input(x) => {fxd = Some(x); if binary{sidx+=1;} break;},
       _=> return ParamParse::Undefined,
      }
     }
    }
    if binary{
    ParamParse::Param(Parameters::new(def_style,def_func,fxd,sidx,binary))
    }
    else{
    ParamParse::Param(Parameters::new(def_style,def_func,fxd,sidx,binary))
    }
 }
/**/

fn parse_integer(input: &str, rad: Radix) -> Value{
   let k = input.trim();
   
   if k == "q" || k=="quit"||k=="exit"{
      return Value::Quit;
   }
   
   match u128::from_str_radix(k,rad as u32){
       Ok(x) => Value::Input(x),
       Err(_) => Value::ParseError(k.to_string()),
   }
}


pub(crate) fn pipe(params: Parameters){
   use std::io::{stdin,BufRead};
   for line in stdin().lock().lines(){
       match line{
          Ok(l) => {
              match parse_integer(&l,params.style){
                Value::Input(num) => {print!("{}",function_eval(num,params.fixed.unwrap(),params.func));},
                Value::ParseError(error_mess) => {eprintln!("'{}' is not a valid positive integer",error_mess);},
                Value::Quit => {break;},
              }
          }
         Err(error_mess) => {eprintln!("{}",error_mess)} 
        }
   }
}

pub(crate) fn greedy(params: Parameters,env_var: &Vec<String>){
        let offset = 1;
        
        for i in env_var[params.idx+offset..].iter(){
        
           match u128::from_str_radix(i,params.style as u32){
              Ok(x) => print!("{}",function_eval(x,params.fixed.unwrap(),params.func)),
              Err(_) => eprintln!("'{}' is not a valid positive integer",i),
           }
          
        }
}
