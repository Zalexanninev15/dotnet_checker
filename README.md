# Universal .NET components checker

<img src="https://raw.githubusercontent.com/Zalexanninev15/dotnet_checker/main/logo.png" width="128" height="128">

[![](https://img.shields.io/badge/platform-Windows-informational)](https://github.com/Zalexanninev15/dotnet_checker)
[![](https://img.shields.io/badge/written_on-Rust-000000.svg?logo=rust)](https://github.com/Zalexanninev15/dotnet_checker)
[![](https://img.shields.io/github/v/release/Zalexanninev15/dotnet_checker)](https://github.com/Zalexanninev15/dotnet_checker/releases/latest)
[![](https://img.shields.io/github/downloads/Zalexanninev15/dotnet_checker/total.svg)](https://github.com/Zalexanninev15/dotnet_checker/releases)
[![](https://img.shields.io/github/last-commit/Zalexanninev15/dotnet_checker/main.svg)](https://github.com/Zalexanninev15/dotnet_checker/commits/main)
[![](https://img.shields.io/github/stars/Zalexanninev15/dotnet_checker.svg)](https://github.com/Zalexanninev15/dotnet_checker/stargazers)
[![](https://img.shields.io/github/forks/Zalexanninev15/dotnet_checker.svg)](https://github.com/Zalexanninev15/dotnet_checker/network/members)
[![](https://img.shields.io/github/issues/Zalexanninev15/dotnet_checker.svg)](https://github.com/Zalexanninev15/dotnet_checker/issues?q=is%3Aopen+is%3Aissue)
[![](https://img.shields.io/github/issues-closed/Zalexanninev15/dotnet_checker.svg)](https://github.com/Zalexanninev15/dotnet_checker/issues?q=is%3Aissue+is%3Aclosed)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![](https://img.shields.io/badge/Donate-FFDD00.svg?logo=buymeacoffee&logoColor=black)](https://z15.neocities.org/donate)

## Description

Universal .NET-components checker for Windows.

## Features

- Checks all types of installed .NET components: .NET Framework, .NET Core, and .NET
- Outputs quite detailed information, even on installed SDKs!
- Compiled in such a way as to have no dependencies.

## System requirements

* **OS:** Windows 7 or higher.

## Build (with PowerShell)

1. Install all dependencies as Admin (it is recommended to use packages from the [Chocolatey package manager](https://chocolatey.org))

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

Then:

```batch
choco install rust mingw git -y
```

2. Compile the Universal .NET components checker!

```batch
git clone https://github.com/Zalexanninev15/dotnet_checker
cd .\dotnet_checker\
cargo build --release
```

> When compiling, I used an outdated version of Rust, so compilation problems may occur on new versions!
