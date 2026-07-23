# Developer Guide

## Environment

* Primary Language: [Cryptol](https://github.com/GaloisInc/cryptol)
* Secondary Languages: [SAW Script](https://github.com/GaloisInc/saw-script), Rust, YAML
* Integrated Development Environment: Visual Studio Code + Docker
* Source Control: Git

## Quick start

This repository is integrated with [GitHub's
Codespaces](https://github.com/codespaces). Codespaces is a
containerized development platform that, when paired with this
project, provides all the tools needed to develop and test
contributions.

If you'd prefer to develop in a different way, the Dockerfiles used to
build the development images are available in the
[`.github/dockerfiles`](../.github/dockerfiles) directory. They will
likely be helpful for customizing a compatible development
environment.

## Licensing and copyright

* Primary License: [Apache License 2.0](LICENSE)

If possible please try to stick to the [Apache License 2.0](LICENSE)
license when developing for Foundation.

If you are contributing code to the Foundation project, the preferred
way to receive credit/recognition is Git commit authorship. Please
ensure your Git credentials are properly linked to your GitHub account
so you appear as a Foundation contributor on GitHub. We do not put
authors' names directly in specifications or other artifacts.

## Style

  * Cryptol: Our style guide for Cryptol exists in the [docs](../docs).
  * SAW: We do not yet have a style guide for SAW scripts.
  * Rust: Rust reference implementations and (Cryptol) foreign
    function interfaces must adhere to `rust-lang.org`'s [style
    guide](https://doc.rust-lang.org/stable/style-guide/). Exceptions,
    such as naming conventions that facilitate correspondence between
    Rust and the corresponding specification or Cryptol, should be
    documented as accepted via the judicious use of Rust's lint
    attributes (ex. `#[allow(non_camel_case_types)]`). All Rust
    projects included here must pass `cargo fmt --all -- --check` as
    well as `cargo clippy --all-targets --all-features --
    -Dclippy::all -Dclippy::pedantic`.

## Running tests

This project contains two development workflows:
  * Cryptol: This includes the development of Cryptol specifications, testing of those specifications, and reference implementations (currently in Rust) that enable efficient computation (via Cryptol's Foreign Function Interface (FFI)) where desired.
  * SAW: This includes the development of SAW specifications that provide assurance guarantees of Cryptol specifications and reference implementations.

To support these workflows, two virtualized development environments are available (clicking either link will create a development environment in GitHub's [Codespaces](https://github.com/codespaces):
   * [Cryptol Codespace](https://codespaces.new/NationalSecurityAgency/foundation?devcontainer_path=.devcontainer%2Fcryptol-dev%2Fdevcontainer.json)
   * [SAW Codespace](https://codespaces.new/NationalSecurityAgency/foundation?devcontainer_path=.devcontainer%2Fsaw-dev%2Fdevcontainer.json)

The assurance tests (invariants) enforced via GitHub's Continuous Integration system are collected in [a local GitHub workflow](../.github/workflows/assurance-main.yml).

### Cryptol

To execute Cryptol-focused assurance tests during development, first open the [Cryptol Codespace](https://codespaces.new/NationalSecurityAgency/foundation?devcontainer_path=.devcontainer%2Fcryptol-dev%2Fdevcontainer.json). Then, set the `CRYPTOLPATH` environment variable to point to the `foundation` project and start up [`cryptol-remote-api`](https://github.com/GaloisInc/cryptol/tree/master/cryptol-remote-api), a JSON RPC server that provides a programmatic interface to Cryptol from general purpose programming languages such as Python.

```console
$ export CRYPTOLPATH=/workspaces/foundation
$ start-cryptol-remote-api
```

Next, execute any of the Cryptol-focused assurance tests found in the [CI](../.ci) directory, for example, the following test ensures that all all of this project's Cryptol files load without error:

```console
$ python3 .ci/load-all-cry-files.py
```

If desired, each of this project's Rust reference implementations can be built by running the following command.

```console
$ .ci/build-rust-ffi.sh
```

This will enable Cryptol specifications with FFI components to make use of performant Rust implementations, decreasing (in many cases) the time it takes to run tests.

#### TODO running tests via `:check-docstrings`

### SAW

To execute SAW-focused assurance tests during development, first open the [SAW Codespace](https://codespaces.new/NationalSecurityAgency/foundation?devcontainer_path=.devcontainer%2Fsaw-dev%2Fdevcontainer.json). Then, set the `CRYPTOLPATH` environment variable to point to the `foundation` project and start up [`saw-remote-api](https://github.com/GaloisInc/saw-script/tree/master/saw-remote-api), a JSON RPC server that provides a programmatic interface to SAW from general purpose programming languages such as Python.

```console
$ export CRYPTOLPATH=/workspaces/foundation
$ start-saw-remote-api
```

Next, execute any of the SAW-focused assurance tests found in the [CI](../.ci) directory, for example, the following test proves various properties about the provided Cryptol specifications:

```console
$ .ci/run-top-level-saw-files.sh
```

If desired, each of this project's Rust reference implementations can be symbolically executed by running the following command.

```console
$ .ci/build-rust-saw-artifacts.sh
```

This enables the verification of Rust reference implementations against Cryptol specifications, increasing the assurance of such implementations. Rust verification can be run by issuing the following command:

```console
$ .ci/run-saw-python-files.sh
```

## Architecture overview

Here will go the methodology for the repository structure

## Developing a new algorithm

Here will go a quick-start for creating a new set of Cryptol specs,
reference implementations, assurance artifacts, etc.
