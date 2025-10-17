use machine_factor::{factorize_128,Factorization128};

struct Signed(bool,u128);

impl Signed{
  const fn sub(a: Self, b: Self) -> Self{
        match (a.0,b.0){
        (false,false) => {
           if b.1 >= a.1{
              Signed(true,b.1-a.1)
           }
           else{
             Signed(false,a.1-b.1)
           }
        },
        (false,true) => {
           Signed(false,a.1+b.1)
        }
        (true,true) => {
          if b.1 >= a.1{
            Signed(false,b.1-a.1)
          }
          else{
            Signed(true,a.1-b.1)
          }
        }
        (true,false) => {
            Signed(true,a.1+b.1)
        }
        }
   }
   
   const fn prod(a: Self, quo: u128) -> Self{
       Self(a.0,a.1*quo)
   }
   
   const fn residue(a: Self, res: u128) -> u128{
      if !a.0{
         let k = a.1%res;
         res-k
      }
      else{
         a.1%res
      }
   }
}

pub(crate) fn residue_eval(x: u128, n: u128) -> String{
    if n==0{
       return "DNE".to_string();
    }
    (x%n).to_string()
}

pub(crate) fn mul_inverse(x: u128, n: u128) -> (u128,u128){
         let mut gcd : u128 =x;
         let mut new_r : u128 =n;
         let mut bezout_1 : Signed = Signed(true,1);
         let mut new_s : Signed = Signed(true,0);
          
        while new_r != 0 {
    
        let quotient =gcd/new_r;
        let temp_r : u128 =new_r;
        let prod = quotient*new_r;
   
        new_r=gcd-prod;
        gcd=temp_r;
  
        let  temp=new_s;
        let temp2 = Signed(temp.0,temp.1);
        new_s=Signed::sub(bezout_1,Signed::prod(temp,quotient));
        bezout_1=temp2;
        }
        (gcd,Signed::residue(bezout_1,n))
} 

 

 fn odd_pow(x: u128, pow: u128, n: u128) -> u128{
    // Handle n < 2^64 using 64-bit functions, this is necessary as machine-prime's 
    // 128-bit montgomery transform "incorrectly" maps n < 2^64
    if n < 1u128<<64{
       debug_assert!(pow < 1u128<<64);
       let b = (x%n) as u64;
       let nsmall = n as u64;
       let mut base = machine_prime::to_mont(b,nsmall);
       let one = machine_prime::one_mont(nsmall);
       let inv = machine_prime::mul_inv2(nsmall);

       base = machine_prime::mont_pow(base,one,pow as u64,nsmall,inv);
       base = base.wrapping_mul(inv);
       base = (base as u128).wrapping_mul(nsmall as u128).wrapping_shr(64) as u64;
       base.wrapping_neg().wrapping_add(nsmall) as u128
    }
    else{
    let one = machine_prime::one_mont_128(n);
    let mut  base = machine_prime::to_mont_128(x%n,n);
    let inv = machine_prime::mul_inv2_128(n);
    base = machine_prime::mont_pow_128(base,one,pow,n,inv);
    base = base.wrapping_mul(inv);
    base = machine_prime::u256prod_lo(base,n);
    base.wrapping_neg().wrapping_add(n)
  }

}

 fn even_pow(x: u128, mut pow: u128, n: u128) -> u128{
       let mut z = 1;
       let mut base = x;
       while pow > 1 {
           if pow & 1 == 0 {
               base = base.wrapping_mul(base)&n;
                pow >>= 1;
            } else {
                 z = base.wrapping_mul(z)&n;
              base = base.wrapping_mul(base)&n;
               pow = (pow - 1) >> 1
        }
    }
      base.wrapping_mul(z)&n
}


pub(crate) fn exp_residue(a: u128, p: u128, n: u128) -> u128{
    
    if n&1==0{
        let k = n.trailing_zeros() as u128;
        let s = n >> k;

        let reducer = (1 << k) - 1; // A shorthand for arithmetic over Z[2k]

        let k_rem = even_pow(a,p,reducer);

        let s_rem = odd_pow(a,p,s);
        
        let s_inv = machine_prime::mul_inv2_128(s)&reducer;
    
        let y = k_rem.wrapping_sub(s_rem).wrapping_mul(s_inv) & reducer;

        s.wrapping_mul(y).wrapping_add(s_rem)
    }
    else{
      odd_pow(a,p,n)
    }
}

const fn ord_2(a: u128, p: u128) -> u128{
        let modulo = (1u128<<p)-1;
        let mut b = a&modulo;
   
         if b == 1{
            return 1;
         }
         let mut idx = 0;
         while idx < p{
         b = b.wrapping_mul(b)&modulo;
         if b == 1{
           return 1<<idx;
         }
         idx+=1;
        }
         return p;
      }
 

// Given ord(a,p)  calculate ord(a,p^n)
     fn pp_ord(a: u128, b: u128, p: u128, e: u32) -> u128{
       let mut idx = 0;
       
       while idx < e+1{
          if exp_residue(a,b*p.pow(idx),p.pow(e)) ==1{
             return b*p.pow(idx);
           }
           idx+=1;
   }
     b*p.pow(e)
    }
    

 fn p_ord(a: u128, p: u128) -> u128{
   
   let fctr = factorize_128(p-1);
   
   let mut m = p-1;
   let mut idx = 0;
   
   while idx < fctr.len{
   
     let factor = fctr.factors[idx];
     let mut inner = 0;
     
     while inner < fctr.powers[idx]{
          if exp_residue(a,m/factor,p) == 1{
            m = m/factor;
          }
          else{
            break;
          }
          inner+=1;
     }
     idx+=1;
    }
   m
  }

 // Multiplicative order of element a in ring

pub(crate) fn ord_eval(a: u128,ring: u128) -> String{
      if gcd(a,ring) != 1{
       return "Does Not Exist".to_string();
      }
      if ring == 0{
         return "Infinite".to_string();
      }
    let fctr = factorize_128(ring);
    
    let mut fullord = 1u128;
    let mut idx = 0;
    
    while idx < fctr.len{

     let factor = fctr.factors[idx];
     let power = fctr.powers[idx];
     
     let mut ord : u128;
     
      if factor == 2{
         ord = ord_2(a,power as u128);
      }
      else{
        ord = p_ord(a,factor);
        if power > 1{
           ord=pp_ord(a,ord,factor,power as u32);
        }
      }
       fullord = lcm(fullord,ord);
       idx+=1;
    }
    fullord.to_string()
 }
 
 pub(crate) fn fermat(base: u128, n: u128) -> bool{
     if n < 2{
        return false;
     }
     exp_residue(base,n-1,n)==1
 }
 
 pub(crate) fn strong_fermat(base: u128, n: u128) -> bool{
     if n < 2{
         return false;
     }   
     if n&1 == 0{
       exp_residue(base,n-1,n)==1
     }
     else{
      
        let p_minus = n.wrapping_sub(1);
        let tzc = p_minus.trailing_zeros();
      
        let bmont = machine_prime::to_mont_128(base,n);
        let inv = machine_prime::mul_inv2_128(n);
        
        let one = machine_prime::one_mont_128(n);
        let oneinv = machine_prime::mont_sub_128(n, one, n);
       
    
        machine_prime::strong_fermat_128(n,tzc,bmont,one,oneinv,inv)
     }
     
 } 

pub fn max_factor(x: u128) -> String{
    if x == 0{
       return "Infinitely large".to_string();
   }
   
   if x == 1{
      return "1".to_string();
   }
   
   let f = factorize_128(x);
   
   f.factors.iter().max().unwrap().to_string() 
}

// Return Infinite for 0 and 1 for 1
pub fn math_style(x: u128) -> String{
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
pub fn sigma_eval(x: u128) -> String{
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


pub fn euler_eval(x: u128) -> u128{
    if x == 1{
       return 1u128;
    }
    let mut numerator = 1u128;
    let mut denominator = 1u128;
    
    let fctr = factorize_128(x);
    
    for idx in 0..fctr.len{
      numerator*=fctr.factors[idx]-1;
      denominator*=fctr.factors[idx];
    }
    (x/denominator)*numerator
}

pub fn gcd(x: u128, y: u128) -> u128{
    if x == 0{
       return y;
    }
    if y == 0{
      return x;
    }
    let xtwo = x.trailing_zeros();
    let ytwo = y.trailing_zeros();
    let mut min = xtwo;

    if xtwo > ytwo{
       min = ytwo;
    }
    machine_factor::partial_gcd_128(x>>xtwo,y>>ytwo)<<min
}

pub fn lcm(x: u128, y: u128) -> u128{
    (x/gcd(x,y))*y
}

pub fn checked_lcm(x: u128, y: u128) -> Option<u128>{
    let g = gcd(x,y);
    let xreduced = x/g;
    let (prod,flag) = xreduced.overflowing_mul(y);
    if flag{
      return None;
    }
    return Some(prod)
}

pub fn nth_root(x: u128, n: u8) -> u128{

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
     return machine_prime::is_prime_128(x);
   }
   let root = nth_root(x,n);
   if root.pow(n as u32) == x && machine_prime::is_prime_128(root){
      return true;
   }
   return false;
}

pub fn cyclic_eval(mut x: u128) -> bool{
   if x == 0{
      return false;
   }
   if x < 8{
      return true;
   }
   let tzc = x.trailing_zeros();
   
   if tzc==1{
      x>>=1;
   }
   if tzc>1{
      return false;
   }
   for i in machine_factor::PRIMES_101{
       let powest = x.ilog2()/i.ilog2();
       if x==(i as u128).pow(powest){
          return true;
       }
   }
   for i in 1..15{
      if is_prime_power(x,i){
         return true;
      }
   }
   false
}

    fn decomp(x: u128) -> (u32,u128){
       let xminus = x-1;
       let twofactor = xminus.trailing_zeros();
       (twofactor,xminus>>twofactor)
    }
    
pub fn euler_factorization(x: u128,fctr: &Factorization128) -> u128{

    let mut numerator = 1u128;
    let mut denominator = 1u128;
    
    for idx in 0..fctr.len{
      numerator*=fctr.factors[idx]-1;
      denominator*=fctr.factors[idx];
    }
    
    (x/denominator)*numerator
}

pub fn liar_factorization(x: u128,fctr: Factorization128) -> u128{
      let xminus = x-1;
      let mut prod = 1;
      
      for i in 0..fctr.len{
        let factor = fctr.factors[i];
        if factor == 2{
           continue;
        }
         prod*=gcd(xminus,factor-1);
      }
      prod
}

pub fn strongliar_factorization(x: u128,fctr: Factorization128) -> u128{
      let xd = decomp(x).1;
      let mut mine = 128;
      let m = fctr.len as u32;

      let mut prod = 1;
      
     for i in 0..m{
        let p = fctr.factors[i as usize];
        let (pe,pd) = decomp(p);
        if pe < mine{
           mine = pe;
        }
        prod*=gcd(xd,pd);
     }
     
   let denom = 2u128.pow(m)-1;
   let numer = 2u128.pow(m*mine)-1;
   let multiplicand = (numer/denom)+1;
    prod*multiplicand
}

pub fn ur_func(x: u128) -> (u128,u128){
    let fctr  = factorize_128(x);
    let unit_order = euler_factorization(x,&fctr);
    if x&1==0{
       let liars = liar_factorization(x,fctr);
       let cofactor = gcd(unit_order,liars);
       (liars/cofactor,unit_order/cofactor)
    }
    else{
      let liars = strongliar_factorization(x,fctr);
      let cofactor = gcd(unit_order,liars);
      (liars/cofactor,unit_order/cofactor)
    }
    
}

pub fn unit_ratio(x: u128) -> String{
    let fctr  = factorize_128(x);
    let unit_order = euler_factorization(x,&fctr);
    if x < 2{
       return "0".to_string();
    }
    let (l,u) = ur_func(x);
    format!("{}/{}",l,u)
    /*
    if x&1==0{
       let liars = liar_factorization(x,fctr);
      let cofactor = gcd(unit_order,liars);
      format!("{}/{}",liars/cofactor,unit_order/cofactor)
    } else{
      let liars = strongliar_factorization(x,fctr);
      let cofactor = gcd(unit_order,liars);
      format!("{}/{}",liars/cofactor,unit_order/cofactor)
    }
    */
}

pub fn unit_ratio_d(x: u128) -> f64{
   if x < 2{
      return 0f64;
   }
   let (l,u) = ur_func(x);
   l as f64/u as f64
}

pub fn frobenius_idx(x: u128) -> Option<i64>{
    if jacobi(x-1,x)==-1{
       return Some(-1i64);
    }
    if jacobi(2,x) == -1{
       return Some(2);
    }
    let sqrt = x.isqrt();
    if sqrt*sqrt == x{
       return None;
    }
    
    let mut start = 3u128;
    
    loop{
      if jacobi(start,x)==-1{
         return Some(start as i64);
      }
      if start >= x {
         return None;
      }
      start+=2;
    }
    
}

pub fn fstring(x: u128) -> String{
    if x < 3{
      return "DNE".to_string()
    }
    match frobenius_idx(x){
      Some(res) => res.to_string(),
      None => "DNE".to_string(),
    }
}
    
pub fn liar_eval(x: u128) -> u128{
      if x < 5 || machine_prime::is_prime_128(x){
         return 0u128;
      }
      let xminus = x-1;
      let tzc = x.trailing_zeros();
      // Eliminate factor of 2
      let fctr = factorize_128(x>>tzc);
      let mut prod = 1;
      
      for i in 0..fctr.len{
         prod*=gcd(xminus,fctr.factors[i]-1);
      }
      prod-1
}    

/// Count of strong fermat liars minus 1
pub fn strong_liar_eval(x: u128) -> u128{

    if x < 5 || machine_prime::is_prime_128(x){
       return 0;
    }
      // if x \in 2Z 
    if x&1==0{
      liar_eval(x)
    }
    else{
      let xd = decomp(x).1;
      let fctr = factorize_128(x);
      let mut mine = 128;
      let m = fctr.len as u32;

      let mut prod = 1;
     for i in 0..m{
        let p = fctr.factors[i as usize];
        let (pe,pd) = decomp(p);
        if pe < mine{
           mine = pe;
        }
        prod*=gcd(xd,pd);
     }
     
   let denom = 2u128.pow(m)-1;
   let numer = 2u128.pow(m*mine)-1;
   let multiplicand = (numer/denom)+1;
    prod*multiplicand-1
  }
}

pub fn omega_eval(x: u128) -> String{
   if x == 0{
      return "Infinite".to_string();
   }
   if x == 1{
      return "0".to_string();
   }
   factorize_128(x).len.to_string()
}


pub fn omegatwo_eval(x: u128) -> String{
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

pub fn jacobi(a: u128,k: u128) -> i8{
        
        let mut n = a;
        let mut p = k;
        let mut t = 1i8;
        n %= p;

        while n != 0 {
            let zeros = n.trailing_zeros();
            n >>= zeros;

            if (p % 8 == 3 || p % 8 == 5) && (zeros % 2 == 1) {
                t = -t
            }
            
            std::mem::swap(&mut n, &mut p);
            
            if n % 4 == 3 && p % 4 == 3 {
                t = -t;
            }
            n %= p;
        }

        if p == 1 {
            t
        } else {
            0
        }
}

pub(crate) fn kronecker(a: u128, k: u128) -> i8{
   if k==0 && a==1{
      return 1i8;
   }
   if k==0{
      return 0i8;
   }
   if k&1==1{
      jacobi(a,k)
   }
   else{
      let tzc = k.trailing_zeros();
      let odd = k>>tzc;
      let res = a%8;
      let mut even_component = 1i8;
      if (res == 3 || res == 5) && tzc&1==1{
         even_component = -1i8;
      }
      if a&1==0{
        even_component = 0i8;
      }
      even_component*jacobi(a,odd)
   }
}


pub fn mobius_eval(x:u128) -> String{
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

pub fn liouville_eval(x: u128) -> String{
   if x == 1{
      return "1".to_string();
   }
   let fctr = factorize_128(x);
   if fctr.len&1==0{
      return "1".to_string();
   }
    "-1".to_string()
}


pub fn exponent_eval(x: u128) -> String{
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


pub fn gnu_style(x: u128) -> String{
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

