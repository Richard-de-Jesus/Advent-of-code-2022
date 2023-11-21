j# Advent-of-Code-2022
Each day is going to have a archive beside it's root directory, to show the improvement of my Rust code.
Gonna list my experience from each day and what i learned. Later i am going to do the solutions again in PHP, to learn it and see the difference in expressiveness between the 2 languages. 

I like low-level code for fun, but need PHP for university.
# Day 01
It was very hard, spent 1 hour trying many naive solutions, later learned more about Rust std libray, discovered that i could use the include_str macro, since the file is know at compile time, then using a map inside a map. 
On 2nd part, found functions to reverse a iterator and take a number of elements.
Learned a lot, i was learning C before Rust, so i knew nothing about functional programming.
# Day 02
Not so hard today, learned how to get a char out of a string and the  ASCII value of a char.
Also discovered that i can match a tuple, AI helped me here, i lack creativity sometimes.
# Day 03
Discovered the chunk function to creat sub-vectors in a vector, had a problem that the program was panicing on unwrap, after the find method, found out that the program was trying to read after the last line, just inserted a default value of 0 in unwrap_or, problem solved.
