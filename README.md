# Type-Safe Sandboxing of Memory-Unsafe C/C++ Libraries for Rust Programs

My master's thesis project, advised by [Dr. Yeongjin Jang](https://unexploitable.systems) at
Oregon State University. For further information, see my [paper](/paper/thesis.pdf) and
[slides](/paper/slides.pdf).

Abstract:

> As the software engineering industry adopts memory-safe programming languages for security- and
performance-sensitive systems programming, developers face a need for interoperability between
memory-safe and memory-unsafe codebases, without compromising on either security or performance.
We present a technique for sandboxing unsafe dependencies within a safe program, using hardware
features such as Intel's Memory Protection Keys to prevent memory corruption bugs within unsafe
code from affecting the safe portions of the program. We use the type system and metaprogramming
capabilities of the Rust programming language to create a low-overhead, ergonomic, and type- and
memory-safe interface allowing the safe portions of the program to freely interact with the
unsafe portions, without risk of inducing undefined behavior in memory-safe code.
