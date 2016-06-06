# cpp-new
A small Rust script that will create a C++ project in a Cargo-like setup.

Platforms
---

This is tested on MSYS 2 on Windows 10, it likely will not work on other MSYS shells or versions of Windows (currently the Makefile checks against `NT10.0` which I assume means Windows 10.0, I'm not sure how to generalize the check). It should also work properly on Darwin and MOST flavors of Windows.

When to Use
---

This is suitable for small-medium projects and quick drafting/scratch work (i.e. playing with a new API). For large, team, distributed, or redistributable projects, some more sophisticated form of auto makefile generation and dependency management such as `cmake` is recommended.

Generated Project Structure
---

Running `cpp-new project_name` will,  if able, generate a new project with the following structure:

* A Makefile in the folder `project_name` (relative to `.` where the script was activated)
* Three folders, `mingw`, `darwin`, and `linux` in `project_name/libs` which are linked with `-L` by the makefile.
* An example header called "common.hpp" in `project_name/include`. This directory is included with -I.
* A source file called `main.cxx` in the `project_name/src` directory. The default file extension generated can be changed with a flag, e.x. `cpp-new project_name --ext=cpp` will generate `main.cpp` instead. Like `cargo new --bin`, this is set up to print "Hello, world!". Trigger warning for K&R style braces. Subdirectories of `src` are supported.
* Upon running `make` it will generate the executable `project_name[.exe on MSYS]` in `target/release`. By default `release` is compiled with `-O3`. A debug build compiled with `-g` and `-O0` can be generated with `make debug`, and will be placed in `target/debug`. (I realize this is inverted from Cargo's behavior; I could be persuaded to change it).
* If it matters, by default `.o` and some dependency tracking files (`.dep`) are placed in `target/*/build`.
 
What you need to do
---

The top area of the Makefile contains `COMPILE_FLAGS` and `LINK_FLAGS`, edit these for compiler features (`-std=c++11`, `-fopenmp` etc), and links (`-lgl`, etc). By default, these are `COMPILE_FLAGS=-std=c++11 -Wall -Wextra` and no linkage.

In addition, `RCOMPILE_FLAGS` and `DCOMPILE_FLAGS` are `COMPILE_FLAGS` that correspond to release and debug builds, respectively. By default, `RCOMPILE_FLAGS` compiles all files with `NDEBUG` defined and `-O3`, while `DCOMPILE_FLAGS` defines `DEBUG`, `-O0` and `-g` (debugging symbols).

What this does *not* do
---

Remote dependency management, `make run` or similar, git initialization (this is TODO, it does generate a .gitignore for you, based on Github's default C++ .gitignore), non-executable projects.

Dependencies and Other Licensing Info
---

Makefile was adapted from [GenericMakefile](https://github.com/mbcrawfo/GenericMakefile), and is thus also subject to its MIT license.

> The MIT License (MIT)

> Copyright (c) 2014 Michael Crawford

> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the "Software"), to deal
> in the Software without restriction, including without limitation the rights
> to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
> copies of the Software, and to permit persons to whom the Software is
> furnished to do so, subject to the following conditions:

> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.

> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.
