# ucas-iclass

[![GitHub License](https://img.shields.io/github/license/PRO-2684/ucas-iclass?logo=opensourceinitiative)](https://github.com/PRO-2684/ucas-iclass/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/ucas-iclass/release.yml?logo=githubactions)](https://github.com/PRO-2684/ucas-iclass/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/ucas-iclass?logo=githubactions)](https://github.com/PRO-2684/ucas-iclass/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/ucas-iclass/total?logo=github)](https://github.com/PRO-2684/ucas-iclass/releases)
[![Crates.io Version](https://img.shields.io/crates/v/ucas-iclass?logo=rust)](https://crates.io/crates/ucas-iclass)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/ucas-iclass?logo=rust)](https://crates.io/crates/ucas-iclass)
[![docs.rs](https://img.shields.io/docsrs/ucas-iclass?logo=rust)](https://docs.rs/ucas-iclass)

iClass API for UCAS. Should also work on other schools, but not guaranteed to work.

## ⚙️ Automatic Releases Setup

1. [Create a new GitHub repository](https://github.com/new) with the name `ucas-iclass` and push this generated project to it.
2. Enable Actions for the repository, and grant "Read and write permissions" to the workflow [here](https://github.com/PRO-2684/ucas-iclass/settings/actions).
3. [Generate an API token on crates.io](https://crates.io/settings/tokens/new), with the following setup:

    - `Name`: `ucas-iclass`
    - `Expiration`: `No expiration`
    - `Scopes`: `publish-new`, `publish-update`
    - `Crates`: `ucas-iclass`

4. [Add a repository secret](https://github.com/PRO-2684/ucas-iclass/settings/secrets/actions/new) named `CARGO_TOKEN` with the generated token as its value.
5. Consider removing this section and updating this README with your own project information.

[Trusted Publishing](https://crates.io/docs/trusted-publishing) is a recent feature added to crates.io. To utilize it, first make sure you've already successfully published the crate. Then, follow these steps:

1. Configuring Trusted Publishing - see the section on [crates.io documentation](https://crates.io/docs/trusted-publishing#Configuring-Trusted-Publishing:~:text=Configuring%20Trusted%20Publishing).
2. Modify `.github/workflows/release.yml` like

## 📥 Installation

### Using [`binstall`](https://github.com/cargo-bins/cargo-binstall)

```shell
cargo binstall ucas-iclass
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/ucas-iclass/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```shell
cargo install ucas-iclass
```

## 💡 Examples

TODO

## 📖 Usage

TODO

## 🎉 Credits

- https://github.com/fontlos/buaa-api/
- https://github.com/zeroduhyy/iclass_buaa/
