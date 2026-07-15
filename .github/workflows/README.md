# GitHub Workflows

## Infrastructure

Workflows prefixed with `infrastructure` contain GitHub Actions for
building the suite of Docker images needed to maintain assurance of
the various components of this repository (specifications, reference
implementations, etc.). Also included here are workflows for building
a development Docker image suitable for use with a containerized
development environment.

### Updating the Infrastructure

These workflows may be started manually, for example, when a new
version of some tool (such as Cryptol) is released. Before running
these workflows on the `main` branch, they should first be tested in a
new branch and any issues resolved in a pull request to the point that
all CI tests pass.

## Release

Workflows prefixed with `release` contain GitHub Actions for building
Docker images containing the `foundation` project as well as tooling
needed to interact with the project in the various workflows here that
create assurance artifacts.

## Assurance

Workflows prefixed with `assurance` contain GitHub Actions that levy
invariants on and create assurance artifacts from the project.

## Cleanup

Workflows prefixed with `cleanup` contain GitHub Actions for cleaning
up the repository.

### Registry

All images built here are stored in [GitHub's container registry for
this `foundation`
project](https://github.com/orgs/NationalSecurityAgency/packages?tab=packages&q=foundation).
