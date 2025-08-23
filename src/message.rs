pub(crate) static HELP : &str = "
Usage: factor [OPTION,OPTION,FIXED] x1,x2,..

  Calculates number-theoretic functions for 
  all the integers provided in the commandline.
  If no arguments are provided then integers are
  read from stdin until q,quit or exit is read.
  Functions labeled as '2-ary' accept a constant
  integer, and read the second argument from stdin.
  All functions have a no-repeat variant callable 
  with {name}-nr, e.g --prime-nr. Functions that 
  have {-filter} return boolean values and can 
  filter to only print the true values.

Options:
  
 Input:
 
  --hex         Reads input as hexadecimal
  --oct         Reads input as octal
  --bin         Reads input as binary
 
 Factorisation:
 
  --no-repeat   Outputs only the factorisation, without repeating input
  --gnu         Outputs GNU factor format
  --gnu-nr      Outputs GNU factor format without repeating input
 
 1-ary:
 
  --cyclic{-filter} Determines if Z/nZ is cyclic
  --euler           Euler totient function, the cardinality of unit subgroup
  --exp             Least exponent function
  --{strong-}liar   Counts the number of fermat liars to N
  --liouville       Liouville function
  --mobius          Mobius function
  --omega{-m}       Counts the number of prime factors of N; with multiplicity
  --prime{-filter}  Determines primality of N
  --sigma           Number of divisors
  
 2-ary:
  
  --gcd              Greatest common divisor to FIXED
  --lcm              Least common multiple to FIXED
  --coprime{-filter} Determines if N is coprime to FIXED
  --kronecker        Kronecker symbol N to FIXED
  --order{-swap}     FIXED^k mod N = 1
  --inverse{-swap}   FIXED^-1 mod N
  --fermat{-filter}  Fermat pseudoprime to base FIXED
  --strong{-filter}  Strong fermat pseudoprime to base FIXED
  
 0-ary:
 
  --about       Returns the version and library about information
  --help        This help page
";

pub(crate) static ABOUT : &str = "

A number-theoretic utility, that is limited to 128-bit.

Functions as a replacement for GNU factor, with additional functionality.

License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

version 1.5 

Copyright (C) 2025 JASory.
";
