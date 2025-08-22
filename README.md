
  Rust port of GNU Factor for 128-bit integers. Some changes have been made to my preference
  
  1. Evaluation loop exits on the input of "q","quit" or "exit"
  
  2. Hexadecimal, Octal, and binary input is supported with the --hex, --oct and --bin options respectively
  
  3. Factorisations are properly represented using exponentiation and multiplication symbols
  
  4. Supports GNU factor style of formatting, with the provided --gnu flag 
  
  5. Supports formatting without repeating the factor, only outputting the factors.
  
  6. Supports other related unary number theoretic functions like primality testing, 
     euler totient, least group exponent, counting divisors, and others
     
  7. Supports evaluating binary number theoretic function to a constant. Multiplicative orders,
     GCD,Kronecker symbol, fermat test, and the strong fermat test. 
     
  8. Supports filtering out primes,coprimes,and fermat pseudoprimes. These functions are 
     heavily limited by stdin/out speed.
  
  Currently faster than the coreutils Rust port, and  begins to outspeed GNU factor for n > 2^50. 
  
  Most semiprimes will be factored in less than 60 seconds (0.6s on average for 128-bit integers)

  This performance is entirely due to the machine-factor library.
  
  Running `make install`, installs it to /usr/bin and redefines factor to call this program instead. 
  
  For a drop-in replacement of GNU factor edit the .bash_aliases file to `alias factor='/usr/bin/rfactor --gnu'` 
  
  This code is licensed under GPLv3. 

