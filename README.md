# Git in Rust

## Introduction
Git is a distributed version control system that tracks changes in any set of computer files, usually used for coordinating work among programmers who are collaboratively developing source code during software development. Its goals include speed, data integrity, and support for distributed, non-linear workflows (thousands of parallel branches running on different computers).

## Objective
The main objective of this development project consists of the implementation of a Git Client and Server with limited functionalities, following the Git development guides and specifications.

## Commands

- Client: cargo run --bin main
- Server: cargo run --bin server server
- Testing: cargo test -- --test-threads 1

## First Version
The first version includes the following functionalities.

- Repository cloning.
- Basic local commands (hash-object, cat-file, init, status, add, rm, commit, checkout, log, merge y branch) and remote - commands (push, pull, fetch, remote).
- Implementation of a graphical interface.

## Second Version
The second version includes complementary commands: check-ignore, ls-files, ls-tree, show-ref, rebase and tag.

## Authors

Name | Code | Email
------ | ------| -------------
[Movia, Guido Alejandro](https://github.com/gmovia) | 102896 | gmovia@fi.uba.ar
[De Feo, Laura Mailen](https://github.com/ldefeo) | XXXX | ldefeo@fi.uba.a
[Diaz Calixto, Luz](https://github.com/) | XXXX | XXXX@fi.uba.ar
[Moralejo, Agustin](https://github.com/) | XXXX | XXXX@fi.uba.ar
