# csc340_assignment4
Sam Calise and Claudia Deverdits

We have received help from multiple TAs during the development of our solution when we ran into out-of-bounds errors during decompression. 

We believe we have correctly implemented all aspects of this program. 

### Program Architecture
Our program reads in a ppm file and trims the image. We then get the Y, Pb, and Pr values and update them for each element of our RGB image that is placed into a vector. We focus on 2x2 grids and find the average values of Pb and Pr. We then use the DCT function to give a, b, c, and d values and then use them in bitpack.rs. 
 We also have functionality to decompress a file where we use bitpack to return the values of a, b, c, d, Pb, and Pr to decimals. We then use an inverse DCT to finish the decompression process.

We spent approximately ~35 hours analyzing each problem within the assignment. 

We spent approximately ~10 hours solving the problems.
