Rohan Kelley

I utilized the hints that were on the assignment sheet, and I used the concepts that were shown and talked about in class.

I believe this successfully compresses an image to a binary file, which can then be decompressed back to a `ppm` file. Running the original image against my compressed and then decompressed image, I get an RMSD of 0.0205 which I think is an acceptable difference. 

The architecture of the program was to make sure to gradually implement each step of the program, testing each part, making the next part, testing the original and the new part together and so on. The first module that I created was to convert RGB pixels to Component Video, and the inverse of this. The next module dealt with packing Component video pixels into 2x2 blocks, and then unpacking 2x2 blocks into component video pixels. The next module did the discrete cosine transformations on the component video data in the 2x2 blocks, and then was able to convert the values in cosine space back to component video. The next module dealt with quantizing the data in cosine space, and then converting those values back to cosine space. Finally, the last module utilized bitpack, and packed the quantized values into 32 bit codewords, which could then be unpacked into quantized values. By setting up my program like this, I was easily able to gradually test each component as I made them and I was able to see if the values in the test cases were behaving correctly when paired with other modules.

I would say I probably spent around 12 hours analyzing the problems in the assignment, as understanding what needed to be done, how to break the problem down, and the importance of making up good test cases were truly the focal points of this assignment. 

Approximately 8 hours were spent solving the implementation of arith.