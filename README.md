# Base64 Encoder and Decoder

Encodes and decodes strings in base64

I wrote two versions, one in rust and one in python. The rust version is roughly 2000% faster and 20% less code. 

If we encrypt a string "Jake Armendariz" with base64 35 times it becomes 235,969 characters. To encrypt and decrypt it takes python 20 times longer than rust

Python: 3m6.764s

Rust:   0m9.655s

I made this for fun, probably not the best demonstration of rust speed, but I thought it was very cool how easy it was to write

To run:
-  ! cargo run "random string"
-  ! python3 test.py "random string"