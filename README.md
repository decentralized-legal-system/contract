# Contract

An exploration of building the contract layer on the IC blockchain.

## 🔥 Goals

1) Role: Provides contracts to the enforcement and auditing layer
2) Resolves: Storing, versioning, and wiring together legal and smart contracts
3) Responsibilities:
    * Creates, Reads, Updates, Deletes, and versions legal contracts.
    * Creates, Reads, Updates, Deletes smart contracts.
    * Establishes a mechanism for combining legal and smart contracts.
    * Send enforcement instructions to the enforcement layer.
    * Sends optional auditing instructions to the auditing layer.
4) Requires: Governance Layer

## 🗺️ Concepts

As with the governance layer, it is very important that a version is finalized and stored.
The standard way of finalizing legal contracts is with a PDF. It would be good if this system can create PDFs with all
the information on the parties, because having a wide variety of links and abstract concepts constituting an agreement
is impossible for anyone to understand. One way to obtain this would be the printing of PDF, with either the text a
variable of what was decided, or links to contracts.

Smart contracts are signed by technical signatures. Signatures could be hashes or cryptographic signatures from specific
identities of KYC or otherwise verified legal person. Also, a digital signature process for machine-to-machine contracts
may be needed.

## 🛠️ Cargo & Make

Cargo works as expected, but in addition to cargo, a makefile exists
that abstracts over several additional tools you may have to install
before all make commands work:

* [clippy](https://github.com/rust-lang/rust-clippy)
* [nextest](https://nexte.st/)
* [outdated](https://github.com/kbknapp/cargo-outdated)
* [udeps](https://crates.io/crates/cargo-udeps)
* [audit](https://crates.io/crates/cargo-audit)

```bash 
    make build          Builds the code base incrementally (fast).
    make check          Checks the code base for security vulnerabilities.
    make fix            Auto-fixes linting issues as reported by cargo and clippy.
    make test           Runs all tests across all crates.
```

## 👷 Development

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd governance/
dfx help
dfx canister --help
```

## 🏠 Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time
you run `dfx deploy`.

## 👨‍💻 Contribution

Contributions are welcomed especially related to documentation, example code, and fixes.
If unsure where to start, open an issue and ask. For more significant code contributions,
please run make test and make check locally before opening a PR.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in deep_causality by you,
shall be licensed under the MIT license without additional terms or conditions.

## 📜 Licence

This project is licensed under the [MIT license](LICENSE).

## 💻 Author

* Marvin Hansen, [Emet-Labs](https://emet-labs.com/).
* Github GPG key ID: 369D5A0B210D39BC
* GPG Fingerprint: 4B18 F7B2 04B9 7A72 967E 663E 369D 5A0B 210D 39BC