// FIXME Add Coprime Add LCM

use crate::math::*;
use crate::functions::{Unary,Binary,Function};
use machine_prime::is_prime_128 as primality;

pub(crate) fn unary_eval(x: u128, op: Unary) -> String{
   
   match op {
      Unary::GNU => x.to_string() + ": " + &gnu_style(x) + "\n",
      Unary::GNUNoRepeat => gnu_style(x) + "\n",
      Unary::Factor => x.to_string() + ": " + &math_style(x) + "\n",
      Unary::FactorNoRepeat => math_style(x) + "\n",
      Unary::Prime => x.to_string() + ": " + &primality(x).to_string() + "\n",
      Unary::PrimeNoRepeat => primality(x).to_string() + "\n",
      Unary::Sigma => x.to_string() + ": " + &sigma_eval(x).to_string() + "\n", 
      Unary::SigmaNoRepeat => sigma_eval(x).to_string() + "\n", 
      Unary::Euler => format!("φ({}): {}\n",x,euler_eval(x)), 
      Unary::EulerNoRepeat => euler_eval(x).to_string() + "\n", 
      Unary::Exp => format!("λ({}): {}\n",x,exponent_eval(x)),
      Unary::ExpNoRepeat => exponent_eval(x) + "\n",
      Unary::Omega => format!("ω({}): {}\n",x,omega_eval(x)),//x.to_string()  + ": "  +&omega_eval(x) + "\n",
      Unary::OmegaNoRepeat => omega_eval(x) + "\n",
      Unary::OmegaTwo => format!("Ω({}): {}\n",x,omegatwo_eval(x)),//x.to_string() + ": " + &omegatwo_eval(x) + "\n",
      Unary::OmegaTwoNoRepeat => omegatwo_eval(x) + "\n",
      Unary::Mobius => format!("μ({}): {}\n",x,mobius_eval(x)),//x.to_string() + ": " +&mobius_eval(x) + "\n",
      Unary::MobiusNoRepeat => mobius_eval(x) + "\n",
      Unary::Liar => x.to_string() + ": " + &liar_eval(x) + "\n",
      Unary::LiarNoRepeat => liar_eval(x) + "\n", 
      Unary::StrongLiar => x.to_string() + ": " + &strong_liar_eval(x) + "\n",
      Unary::StrongLiarNoRepeat => strong_liar_eval(x) + "\n", 
      Unary::Liouville => x.to_string() + ": " + &liouville_eval(x) + "\n",
      Unary::LiouvilleNoRepeat => liouville_eval(x) + "\n",
      Unary::Cyclic => x.to_string() + ": " + &cyclic_eval(x) + "\n",
      Unary::CyclicNoRepeat => cyclic_eval(x) + "\n",
      Unary::PrimeFilter => {if primality(x){x.to_string()+"\n"}else{String::new()}},
      Unary::CyclicFilter => {if cyclic_eval(x)=="true".to_string(){x.to_string()+"\n"}else{String::new()}},
   }
      
  }
  
pub(crate) fn binary_eval(variable: u128,fixed: u128, op: Binary) -> String{
  match op{
   Binary::Gcd => format!("gcd({},{}): {}\n",fixed,variable,gcd(fixed,variable)),
   Binary::GcdNoRepeat => format!("{}\n",gcd(fixed,variable)),
   Binary::CoprimeFilter => {if gcd(variable,fixed)==1{ variable.to_string()+"\n"} else{String::new()}},
   Binary::Coprime => format!("({},{}): {}\n",fixed,variable,gcd(fixed,variable)==1),
   Binary::CoprimeNoRepeat => format!("{}\n",gcd(fixed,variable)==1),
   Binary::Fermat => format!("({},{}): {}\n",fixed,variable,fermat(fixed,variable)&!primality(variable)),
   Binary::StrongFermat => format!("({},{}): {}\n",fixed,variable,strong_fermat(fixed,variable)&!primality(variable)),
   Binary::FermatNoRepeat => format!("{}\n",fermat(fixed,variable)&!primality(variable)),
   Binary::StrongFermatNoRepeat => format!("{}\n",strong_fermat(fixed,variable)&!primality(variable)),
   Binary::FermatFilter => {
                if fermat(fixed%variable,variable)&!primality(variable){
                      variable.to_string()+"\n"
                  } else {
                  String::new()
                  }
                },
            
   Binary::StrongFermatFilter => {
              
              if strong_fermat(fixed%variable,variable)&!primality(variable){
                    variable.to_string()+"\n"
                  } else {
                  String::new()
                  }
                  },
                  
   Binary::Kronecker => format!("kronecker({},{}): {}\n",fixed,variable,kronecker(fixed,variable)),
        
   Binary::KroneckerNoRepeat => format!("{}\n",kronecker(fixed,variable)),

   Binary::Lcm => {
	
	match checked_lcm(fixed,variable){
         Some(x) => format!("lcm({},{}): {}\n",fixed,variable,x),
	 None => "Overflow".to_string()
	}
   }

   Binary::LcmNoRepeat => { 

	match checked_lcm(fixed,variable){
        Some(x) => format!("{}\n",x),
	None => "Overflow".to_string(),   
        }

   }
        
   Binary::Ord => { format!("ord({},{}): {}\n",fixed,variable,ord_eval(fixed,variable))},
   
   Binary::OrdSwap => { format!("ord({},{}): {}\n",variable,fixed,ord_eval(variable,fixed))},

   Binary::OrdSwapNoRepeat => {format!("{}\n",ord_eval(variable,fixed))},
   
   Binary::OrdNoRepeat => {format!("{}\n",ord_eval(fixed,variable))},
   
   Binary::MulInverse => { 
      let (g, bezout) = mul_inverse(fixed,variable);
     if g == 1{
        format!("inverse({},{}): {}\n",fixed,variable,bezout)
     } else {
        format!("inverse({},{}): Does Not Exist\n",fixed,variable)
     }
     }
     
    Binary::MulInverseNoRepeat => { 
      let (g, bezout) = mul_inverse(fixed,variable);
     if g == 1{
        format!("{}\n",bezout)
     } else {
        "Does Not Exist\n".to_string()
     }
     } 
     
     Binary::MulInverseSwap => { 
      let (g, bezout) = mul_inverse(variable,fixed);
     if g == 1{
         format!("inverse({},{}): {}\n",variable,fixed,bezout)
     } else {
         format!("inverse({},{}): Does Not Exist\n",variable,fixed)
     }
     }
     
    Binary::MulInverseSwapNoRepeat => { 
      let (g, bezout) = mul_inverse(variable,fixed);
     if g == 1{
         format!("{}\n",bezout)
     } else {
         "Does Not Exist\n".to_string()
     }
     } 
   
   }
}  
  
pub(crate) fn function_eval(x: u128,y: u128,f: Function) -> String{
   match f{
     Function::ArityOne(operation) => unary_eval(x,operation),
     Function::ArityTwo(operation) => binary_eval(x,y,operation),
   }
}   

