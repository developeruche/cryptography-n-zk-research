# Sumcheck 201 Playground

This section of this research repo is focused on experimenting with the Sumcheck201 algorithm. most of the codes for this crate is copied from ingoyama's implementation. Using the crate to find out which of the sum check's implementation is more efficient (space and time). But time complexity is most important to me. From benches, algorithm one shows to be `30x` faster than algorithm 2. 

I am also using this crate to understand the developers approach to implementing the sumcheck201 algorithms, more like picking their brains :)


I hope with this few points of mine, I have been able to convince and not confuse you, you should not use this code for an thing looking like a production environment!!

Benched marked Sumcheck201 again my Composed Sumcheck Implementation: 
**My composed sumcheck benched results over 2 product Multilinear poly of 10vairables**; 
Composed sum check      time:   [2.7926 ms 2.7991 ms 2.8068 ms]
                        change: [+0.6118% +0.8803% +1.1932%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
  
**Sumcheck201 benched results over 2 product Multilinear poly of 10vairables**; 
Super Sumcheck: Sumcheck201
                        time:   [2.1478 ms 2.1491 ms 2.1505 ms]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) low mild
  2 (2.00%) high mild
  
This does not look to significant, but this would definate add up over time. Going on in this repo's jounery, I would be using this Ingoyama's sumcheck algo adopted to the primitives of this repo. for two reasons; 
1. it is faster than the composed sumcheck implementation
2. it is more flexible. performing sumcheck on a polynomial of this nature "(a * b) + (c * d) lead to the implementation of the MultiComposed Sumcheck algorithm. Same problem occured while working on the product check iop, sumcheck need to be ran of a poly of this structure "(a * b) c + (e * f) g", this multicomposed cumcheck can't handle this structure, this Ingoyama developers did something amazingly flexible, just this sumcheck 201 implementation can handle these two problem structures.
  