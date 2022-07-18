Something something
[this](https://www.youtube.com/watch?v=iSNsgj1OCLA).

This code isn't the prettiest nor is it the most efficient. Most importantly,
all `search_for*` functions initialize a completely new vector on each loop.
I don't know if the compiler can optimize it so that there's only one allocation
for the whole function call instead of one allocation for each loop.

I don't care enough to optimize (or clean up) the code myself... In fact,
I don't actually know whether the code is correct to begin with :D
