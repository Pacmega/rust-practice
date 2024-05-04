## Chapter 2
### Programming a Guessing Game
When describing a crate version, e.g. _0.8.5_ is actually interpreted as _^0.8.5_, meaning any version that is at least 0.8.5 but below 0.9.0. The Cargo.lock file is created by Cargo on first build, and locks in dependency versions until you explicitly update. This can be either via `cargo update`, or by changing the Cargo.toml file. More about Cargo all the way over in chapter 14.
