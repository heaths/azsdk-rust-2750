# Azure SDK for Rust Test

These are test crates for pull request [Azure/azure-sdk-for-rust#2933](https://github.com/Azure/azure-sdk-for-rust/pull/2933) and follow-up scenarios once merged.
They are intentionally not part of a single workspace so we can more easily detect when and why dependencies like `openssl` might've been pulled in.

## Prerequisites

* [Rust](https://www.rust-lang.org)
* [Azure Developer CLI](https://aka.ms/azure-dev)

## Getting Started

Authenticate and deploy resources to run test crates herein:

```sh
azd auth login
azd up
```
