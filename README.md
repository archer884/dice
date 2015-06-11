# dice
Dice library for rust

No, you read that correctly: this seriously exists to support a program I wrote that rolls dice. It's also been featured in one of my [Rust screencasts on YouTube](https://youtu.be/qir8o1vhw9A).

Since the video was posted, I have reorganized the crate somewhat so that it uses more and smaller files. I've also fixed the regular expression used to validate dice expressions (again), but the cool thing is that, because I have a test suite written for the expression parsing, I knew it still worked without having to try it out by hand.

Since making the video, I have integrated this library into my actual rolling program. The source for that is [here](https://github.com/archer884/roll).

## version 1.1

- Adds `.values()` to the dice result struct, which provides a slice into the underlying Vec<u32>

## version 1.2

- Adds `GenFn`, which is a newtype intended to make it a little easier to "implement" the `DiceResultGenerator` trait required by `Dice` to generate new results. 

I see this as an important change to the API because it reduces the friction between the old API and the newone. Where before you could just pass in a function, e.g. `|max| rng.gen_range(0, max) + 1`, it is now possible to do exactly that again provided that you wrap it in like this: `GenFn(|max| rng.gen_range(0, max) + 1)`.

So, basically, it's now possible to have your cake and eat it, too--which is a pretty rare thing, to be honest.
