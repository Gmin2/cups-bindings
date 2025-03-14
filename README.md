# libcups Rust binding exercise

Thank you for offering to work on libcups bindings!

The following is a small exercise so I can judge how much Rust experience you have. It is not intended to be a "filter", just to make sure that this GSOC project is appropriate for your level of experience, so you're not in over your head.

## Instructions

Read the provided `.h` header file. Write a Rust wrapper for the functions in it. I have given you the "skeleton" of a rust project, but you are responsible for designing the Rust API. You do not need to expose all the features of the C code, just enough that someone could print a document using your library.

You can use any dependencies you like as long as they are on crates.io. Please be discerning; you probably do not need 500 dependencies for a project of this size.

You are welcome to read the C code to get a better understanding of the library. However, I will not answer questions about how it works. Part of what I am judging is your ability to read and understand existing code.

To see the type signature of the generated bindings, run `cargo doc --document-private-items --open`.

To build the project, run `make` once to compile the C code. After that, `cargo check` and `cargo build` should work like normal. If that gives you any trouble, please let me know and I will fix the skeleton.

You are not required to write tests, but you will probably find them helpful. You may also want to run your code under [ASan](https://github.com/google/sanitizers/wiki/addresssanitizer) or [Valgrind](https://valgrind.org/docs/manual/quick-start.html).

## Criteria

I am looking for your code to be memory-safe and free of memory leaks.

Note that `Thread::new` is [known to leak](https://github.com/rust-lang/rust/issues/135608); this is not a bug in your code.

Don't worry about having an "idiomatic" API, thread-safety, or anything like that. When you are working on the real libcups codebase, I will mentor you on those issues, but you don't need to think about them for this exercise.

## Time expectations

I have my own solution to this exercise, which took me about 45 minutes to write.

I expect this task to take you 1-5 hours. If it takes more than 10 hours, please stop and send me what you have so far - that indicates that I made this exercise too hard, not that you've done anything wrong.

## Problems that are your responsibility

Segfaults, including segfaults that are due to a bug in the C code. I did not intentionally introduce bugs in the C code, but they may be present, just like the libcups codebase may have latent bugs when you work on it this summer.

## Problems that are my responsibility

If any of the following give you trouble, let me know. They are not intended to be part of the exercise, and a problem indicates that I have written the exercise incorrectly, not that you've done anything wrong.

- A linker error, such as "linking with `cc` failed"
- A compile error for the build script
- A compile error for the .c file

You are free to ask me any questions about this project, and I will make a judgement call of whether or not to answer them. I will not penalize you for asking questions.

Good luck!
