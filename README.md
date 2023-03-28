Running the original image against my compressed and then decompressed image, I get an RMSD of 0.0205 which I think is an acceptable difference. 

Rpeg follows this implementation for compression: 
- Convert RGB pixels to Component Video
- Pack CV pixels into 2x2 blocks
- Discrete Cosine transformation
- Quantization of data in cosine space
- Pack quantized values into 32 bit words


It then does the inverse of all of these operations for the decompression
