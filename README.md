# Foundation

[Foundation][main] is a repository for formal specifications of
cryptographic algorithms and functional assurance artifacts created
and maintained by the [National Security Agency][nsa] Research
Directorate.

If you are a U.S. citizen interested in projects like this, to develop
Foundation and other cybersecurity tools for NSA to help protect our
nation and its allies, consider applying for a [career with
us][career].

## Workflow Status

This repository uses GitHub Actions to add assurance to its
artifacts. The badges here provide the status of those actions.

[![Build images](https://github.com/NationalSecurityAgency/foundation/actions/workflows/infrastructure-build-images.yml/badge.svg)](https://github.com/NationalSecurityAgency/foundation/actions/workflows/infrastructure-build-images.yml)
[![Create assurance artifacts](https://github.com/NationalSecurityAgency/foundation/actions/workflows/assurance-main.yml/badge.svg)](https://github.com/NationalSecurityAgency/foundation/actions/workflows/assurance-main.yml)

## Develop

Two [development container
configurations](.devcontainer/) are maintained in this
repository and provide the tooling (Cryptol, SAW, supported SAT/SMT
solvers, and more) necessary to support its development. These
configurations are compatible with GitHub's
[Codespaces](https://github.com/codespaces), a service that enables
developers to "Spin up fully configured dev environments in the
cloud", as well as [Visual Studio Code's Dev Container
extension](https://code.visualstudio.com/docs/devcontainers/containers). Using
these configurations in a supported development environment (such as
those listed above) is the preferred way to develop in this
repository.

Start developing immediately:
   * [Cryptol Codespace](https://codespaces.new/NationalSecurityAgency/foundation?devcontainer_path=.devcontainer%2Fcryptol-dev%2Fdevcontainer.json)
   * [SAW Codespace](https://codespaces.new/NationalSecurityAgency/foundation?devcontainer_path=.devcontainer%2Fsaw-dev%2Fdevcontainer.json)

## Contribute

If you would like to contribute bug fixes, improvements, and new
features back to Foundation, please take a look at our [Contributor
Guide][contrib] and [Developer Guide][devguide] to see how you can
participate in this open source project.

[nsa]: https://www.nsa.gov
[contrib]: CONTRIBUTING.md
[devguide]: docs/DevGuide.md
[career]: https://www.intelligencecareers.gov/nsa
[releases]: https://github.com/NationalSecurityAgency/foundation/releases
[main]: https://github.com/NationalSecurityAgency/foundation/archive/refs/heads/master.zip
