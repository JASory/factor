
  Rust port of GNU Factor for 128-bit integers. Some changes have been made to my preference
  
  1. Evaluation loop exits on the input of "q","quit" or "exit"
  
  2. Hexadecimal input is supported with the --hex option
  
  3. Factorisations are "properly" formatted using exponentiation and multiplication symbols
  
  Currently faster than the coreutils Rust port, and comparable to GNU factor  running in approximately 1/3t in the case of large semiprimes (the hardest case), although often slower for other "easier" composites.
  
  Most composites will be factored in less than 60 seconds (0.6s on average)

  This performance is entirely due to the machine-factor library.
  
  This code is licensed under GPLv3. 

