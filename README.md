<p align="center">
    <img src="https://cdn-icons-png.flaticon.com/512/77/77649.png" width="150">
    <br>
    <br>
    <img src="https://img.shields.io/badge/language-rust-red.svg" alt="Language">
    <img src="https://img.shields.io/github/stars/ValuONE/disk-name" alt="Stars">
    <img src="https://img.shields.io/github/issues/ValuONE/disk-name" alt="Issues">
    <img src="https://img.shields.io/github/forks/ValuONE/disk-name" alt="Forks">
</p>

<p align="center">
    <a href="#key-features">Key Features</a> •
    <a href="#installation">Installation</a> •
    <a href="#usage">Usage</a> •
    <a href="#supported-platforms">Platforms</a> •
    <a href="#contributors">Contributors</a> •
    <a href="#license">License</a>
</p>

<p>
In a previous project, I needed to retrieve a list of all disk names. While I attempted to use the WinAPI, I found the process cumbersome and disliked the involving unsafe calls. As a result, I decided to implement my own solution using the Rust standard library.</p>

## **Key Features**
- Retrieve all disk names
---
## **Installation**

Use the ``cargo add disk-name`` command or add it manually to your **cargo.toml** file

````toml
[dependencies]

disk-name = "1.0.0"
````

---
## **Usage**

The disk-name crate exposes one function to call.

```rust
fn main() {
    let disk_names: Vec<String> = get_letters();
    println!("{:#?}", disk_names);
}
```

With that simple call all existing letter names are printed.

---
## Supported platforms

1. [x] Windows
2. [ ] Linux
3. [ ] MacOS
---
## Contributors

- [valu](https://github.com/ValuONE)

---

## **License**
[![GitHub license](https://img.shields.io/github/license/pr4k/locate)](https://github.com/pr4k/locate)

- **[MIT license](http://opensource.org/licenses/mit-license.php)**
- Copyright 2024 © ValuONE

---
