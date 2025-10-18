# ucas-iclass

[![GitHub License](https://img.shields.io/github/license/PRO-2684/ucas-iclass?logo=opensourceinitiative)](https://github.com/PRO-2684/ucas-iclass/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/ucas-iclass/release.yml?logo=githubactions)](https://github.com/PRO-2684/ucas-iclass/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/ucas-iclass?logo=githubactions)](https://github.com/PRO-2684/ucas-iclass/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/ucas-iclass/total?logo=github)](https://github.com/PRO-2684/ucas-iclass/releases)
[![Crates.io Version](https://img.shields.io/crates/v/ucas-iclass?logo=rust)](https://crates.io/crates/ucas-iclass)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/ucas-iclass?logo=rust)](https://crates.io/crates/ucas-iclass)
[![docs.rs](https://img.shields.io/docsrs/ucas-iclass?logo=rust)](https://docs.rs/ucas-iclass)

> [!WARNING]
> This repository is for learning purposes only. Use at your own risk.

iClass API for UCAS. Should also work for other schools, but not guaranteed.

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

## 📖 Usage

- 🔑 Login: Login to iClass and save session to a file.

    ```bash
    ucas-iclass login <username> <password> [-s <session-file>]
    ```

- 📖 Courses: List courses in current semester.

    ```bash
    ucas-iclass courses [-s <session-file>]
    ```

- 📃 Schedule: Get schedule for a specific date or week.

    ```bash
    ucas-iclass schedule [-d <date>] [-w] [-s <session-file>]
    ```

- ✅ Checkin: Check-in for a specific schedule by id or uuid.

    ```bash
    ucas-iclass checkin <id_or_uuid> [-s <session-file>]
    ```

## ☑️ TODO

- Customize api root for cli.
- Checkin: Default to current schedule if no `id_or_uuid` provided.

## 🎉 Credits

- https://github.com/fontlos/buaa-api/
- https://github.com/zeroduhyy/iclass_buaa/
