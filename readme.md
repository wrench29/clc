# clc - Count lines of code

A small utility that analyzes a selected directory for
code files, counts lines of code, empty lines and
comment-only lines. By default only Rust code is supported,
but it can support any language you want, all you need
to do is to have `formats.yaml` file located in the folder where
executable file or this project is, and add your languages in it.
There is example `formats.yaml` file in this project, that has
several popular languages configs, you can add whatever language
you want.

## Usage

`$ clc` - analyze files of the directory where you currently are.

`$ clc <path_to_code>` - analyze files of the <path_to_code> directory.
