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
      Unary::MaxFactor => format!("{}: {}\n",x,max_factor(x)),
      Unary::MaxFactorNoRepeat => max_factor(x) + "\n",
      Unary::Prime => x.to_string() + ": " + &primality(x).to_string() + "\n",
      Unary::PrimeNoRepeat => primality(x).to_string() + "\n",
      Unary::Composite => x.to_string() + ": " + &(!primality(x)).to_string() + "\n",
      Unary::CompositeNoRepeat => (!primality(x)).to_string() + "\n",
      Unary::Sigma => format!("σ({}): {}\n",x,sigma_eval(x)),
      Unary::SigmaNoRepeat => sigma_eval(x).to_string() + "\n", 
      Unary::Euler => format!("φ({}): {}\n",x,euler_eval(x)), 
      Unary::EulerNoRepeat => euler_eval(x).to_string() + "\n", 
      Unary::Exp => format!("λ({}): {}\n",x,exponent_eval(x)),
      Unary::ExpNoRepeat => exponent_eval(x) + "\n",
      Unary::Omega => format!("ω({}): {}\n",x,omega_eval(x)),
      Unary::OmegaNoRepeat => omega_eval(x) + "\n",
      Unary::OmegaTwo => format!("Ω({}): {}\n",x,omegatwo_eval(x)),
      Unary::OmegaTwoNoRepeat => omegatwo_eval(x) + "\n",
      Unary::Mobius => format!("μ({}): {}\n",x,mobius_eval(x)),
      Unary::MobiusNoRepeat => mobius_eval(x) + "\n",
      Unary::Liar => format!("L({}): {}\n",x,liar_eval(x)),
      Unary::LiarNoRepeat => liar_eval(x).to_string() + "\n", 
      Unary::StrongLiar => format!("SL({}): {}\n",x,strong_liar_eval(x)),
      Unary::StrongLiarNoRepeat => strong_liar_eval(x).to_string() + "\n", 
      Unary::Liouville => format!("Lλ({}): {}\n",x,liouville_eval(x)),
      Unary::LiouvilleNoRepeat => liouville_eval(x) + "\n",
      Unary::Cyclic => format!("Cyclic({}): {}\n",x,cyclic_eval(x)),
      Unary::CyclicNoRepeat => cyclic_eval(x).to_string() + "\n",
      Unary::PrimeFilter => {if primality(x){x.to_string()+"\n"}else{String::new()}},
      Unary::CompositeFilter => {if !primality(x){x.to_string()+"\n"}else{String::new()}},
      Unary::CyclicFilter => {if cyclic_eval(x){x.to_string()+"\n"}else{String::new()}},
      Unary::UnitRatio => format!("φ({})/SL(n): {}\n",x,unit_ratio(x)),
      Unary::UnitRatioNoRepeat => format!("{}\n",unit_ratio(x)),
      Unary::UnitRatiod => format!("φ({})/SL(n): {}\n",x,unit_ratio_d(x)),
      Unary::UnitRatiodNoRepeat => format!("{}\n",unit_ratio_d(x)),
      Unary::FrobeniusIdx => format!("Fidx({}): {}\n",x,fstring(x)),
      Unary::FrobeniusIdxNoRepeat => format!("{}\n",fstring(x)),
   }
      
  }
  
pub(crate) fn binary_eval(variable: u128,fixed: u128, op: Binary) -> String{
  match op{
   Binary::Residue => format!("{} mod {}: {}\n",fixed,variable,residue_eval(fixed,variable)),
   Binary::ResidueNoRepeat => format!("{}\n",residue_eval(fixed,variable)),
   Binary::ResidueSwap => format!("{} mod {}: {}\n",variable,fixed,residue_eval(variable,fixed)),
   Binary::ResidueSwapNoRepeat => format!("{}\n",residue_eval(variable,fixed)),
   Binary::Gcd => format!("gcd({},{}): {}\n",fixed,variable,gcd(fixed,variable)),
   Binary::GcdNoRepeat => format!("{}\n",gcd(fixed,variable)),
   Binary::CoprimeFilter => {if gcd(variable,fixed)==1{ variable.to_string()+"\n"} else{String::new()}},
   Binary::Coprime => format!("coprime({},{}): {}\n",fixed,variable,gcd(fixed,variable)==1),
   Binary::CoprimeNoRepeat => format!("{}\n",gcd(fixed,variable)==1),
   Binary::Fermat => format!("prp({},{}): {}\n",fixed,variable,fermat(fixed,variable)&!primality(variable)),
   Binary::FermatSwap => format!("prp({},{}): {}\n",variable,fixed,fermat(variable,fixed)&!primality(fixed)),
   Binary::StrongFermat => format!("sprp({},{}): {}\n",fixed,variable,strong_fermat(fixed,variable)&!primality(variable)),
   Binary::StrongFermatSwap => format!("sprp({},{}): {}\n",variable,fixed,strong_fermat(variable,fixed)&!primality(fixed)),
   Binary::FermatNoRepeat => format!("{}\n",fermat(fixed,variable)&!primality(variable)),
   Binary::FermatSwapNoRepeat => format!("{}\n",fermat(variable,fixed)&!primality(fixed)),
   Binary::StrongFermatNoRepeat => format!("{}\n",strong_fermat(fixed,variable)&!primality(variable)),
   Binary::StrongFermatSwapNoRepeat => format!("{}\n",strong_fermat(variable,fixed)&!primality(fixed)),
   Binary::FermatFilter => {
                let mut ring = variable;
                // Avoid division by zero error
                if variable == 0{
                   ring = 1;
                }
                if fermat(fixed%ring,ring)&!primality(ring){
                      ring.to_string()+"\n"
                  } else {
                  String::new()
                  }
                },
            
   Binary::StrongFermatFilter => {
              
              let mut ring = variable;
                // Avoid division by zero error
                if variable == 0{
                   ring = 1;
                }
                
              if strong_fermat(fixed%ring,ring)&!primality(ring){
                    ring.to_string()+"\n"
                  } else {
                  String::new()
                  }
                  },
                  
   Binary::Kronecker => format!("kronecker({},{}): {}\n",fixed,variable,kronecker(fixed,variable)),
        
   Binary::KroneckerNoRepeat => format!("{}\n",kronecker(fixed,variable)),
   
   Binary::KroneckerSwap => format!("kronecker({},{}): {}\n",variable,fixed,kronecker(variable,fixed)),
        
   Binary::KroneckerSwapNoRepeat => format!("{}\n",kronecker(variable,fixed)),

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
     if variable == 0{
        return format!("{}⁻¹ mod {}: Does Not Exist\n",fixed,variable);
     }
      let (g, bezout) = mul_inverse(fixed,variable);
     if g == 1{
        format!("{}⁻¹ mod {}: {}\n",fixed,variable,bezout)
     } else {
        format!("{}⁻¹ mod {}: Does Not Exist\n",fixed,variable)
     }
     }
     
    Binary::MulInverseNoRepeat => { 
     
     if variable == 0{
        return format!("Does Not Exist\n");
     }
    
      let (g, bezout) = mul_inverse(fixed,variable);
     if g == 1{
        format!("{}\n",bezout)
     } else {
        "Does Not Exist\n".to_string()
     }
     } 
     
     Binary::MulInverseSwap => { 
     if fixed == 0{
        return format!("{}⁻¹ mod {}: Does Not Exist\n",variable,fixed);
     }
      let (g, bezout) = mul_inverse(variable,fixed);
     if g == 1{
         format!("{}⁻¹ mod {}: {}\n",variable,fixed,bezout)
     } else {
         format!("{}⁻¹ mod {}: Does Not Exist\n",variable,fixed)
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

