## Chapter 1
`rustc [file]`
- Using the Rust compiler directly, which is usually a _questionable_ idea since Cargo is so easy to use.

`cargo new [project_name] --vcs=none`
- Initialize a new project using Cargo, setting up the basis (literally, since it inits a hello world). Also great because of platform agnostic instructions, instead of `rustc`.
- By default also inits a git repo, `--vcs=none` is especially useful in practice within this repo bc git does not like repos within repos.

`cargo build`
- Take a guess. Default output goes to `target/debug`.

`cargo build --release`
- Build, but compiling with optimizations. _Insert Lightning McQueen 'I am speed' meme._

`cargo run`
- Compile if there are changes, and then straight away run. Cuts away manually running main :)

`cargo check`
- Check your spaghetti to see if it would compile, but do not actually compile it. Especially for bigger projects, this is much faster than a full build and works while developing but not testing yet.
