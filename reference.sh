#Bashscript lines showing applications of factor

# All functions have a variant followed by -nr that only outputs the result, this is for piping and better data handling

#Duplicating GNU factor's behaviour

factor --gnu

#Print without repeating the factor (easier to parse output)

factor --no-repeat

# or GNU style 
factor --gnu-nr

# Display the maximum factor

factor --max

#Return only the maximum factor, useful

factor --max-nr

#All functions have the -nr suffix to avoid repeating the input e.g

factor --euler 17
# φ(17) : 16

factor --euler-nr 17 
# 16

#List primes between 0 and N.

seq 0  N | factor --prime-filter

#Calculating unit subgroup

seq 0 N | factor --coprime-filter N


#Calculating inverses of unit subgroup

seq 0 N | factor --coprime-filter N | factor --inverse-swap N


#Determining the order of the unit group N

factor --euler N

#Determining the minimum exponent of the group N

factor --exp N

#Determining the number of solutions to a^N-1 mod N = 1

factor --fermat-liar N

# Determining the number of strong fermat liars to N

factor --strong-liar N

#Determining the ratio of strong liars to the unit subgroup, in fraction form. 

factor --unit-ratio N

# Same as above but in decimal form, easier for comparison to other ratios

factor --unit-ratio-d N

# List fermat pseudoprimes to witness A. 

seq 0 N | factor --fermat-filter A

# Calculate the range of ratios of the whole set, this is much more useful for sets that aren't just the integers. 
# E.g p*(2p-1) or Carmichael numbers

seq 3 N | factor --unit-ratio-d-nr | sort -n | uniq 

# List strong fermat pseudoprimes to witness A.
seq 0 N | factor --strong-filter A
