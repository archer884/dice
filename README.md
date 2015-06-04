# dice
Dice library for rust

No, you read that correctly: this seriously exists to support a program I wrote that rolls dice. It's also been featured in one of my [Rust screencasts on YouTube](https://youtu.be/qir8o1vhw9A).

Since the video was posted, I have reorganized the crate somewhat so that it uses more and smaller files. I've also fixed the regular expression used to validate dice expressions (again), but the cool thing is that, because I have a test suite written for the expression parsing, I knew it still worked without having to try it out by hand.

Since making the video, I have integrated this library into my actual rolling program. The source for that is [here](https://github.com/archer884/roll).

## version 1.1

- Adds `.values()` to the dice result struct, which provides a slice into the underlying vec
