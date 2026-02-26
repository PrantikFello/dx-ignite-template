```markdown
# 🚀 dx-ignite-template

> **The high-velocity Rust Forge.** Ignite your Dioxus apps with a modular, subtractive foundation.

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![Dioxus](https://img.shields.io/badge/dioxus-0.5-blue.svg?style=for-the-badge)](https://dioxuslabs.com)
[![Architecture](https://img.shields.io/badge/architecture-workspace-blueviolet.svg?style=for-the-badge)](#)

**dx-ignite** is a professional-grade **Cargo Workspace** scaffold for building native cross-platform Rust applications. It is designed to be **subtractive**: start with a full-stack native skeleton and simply prune the crates you don't need to stay lean.

---

## 🏗 The Workspace Blueprint

Logic is strictly decoupled to ensure your design system remains portable and your data-fetching remains unified.

* **`packages/ui`** 🎨 — **The Visual Core.** Scoped CSS Modules, atomic components, and global styling presets.
* **`packages/api`** 🌐 — **The Logic Engine.** Centralized data-fetching, types, and state management.
* **`packages/desktop`** 💻 — **Native Desktop.** High-performance entry point for Windows, macOS, and Linux.
* **`packages/mobile`** 📱 — **Native Mobile.** Ready-to-deploy targets for iOS and Android.

---

## ✨ Why Ignite?

* **⚡ Zero-Friction Scaffolding:** Skip the configuration hell. Just clone, rename, and code.
* **🧩 Subtractive by Design:** Need only a Desktop tool? Delete `mobile/` and `api/` in seconds. No broken dependencies.
* **🎨 Scoped CSS Modules:** Atomic components in `ui/src/components` use local `style.css` modules. No more global style leaks.
* **🦀 Evidence-Based Rust:** Pre-configured with strict `clippy.toml` lints for memory-safe, idiomatic code.
* **📱 True Cross-Platform:** Share 95%+ of your UI and logic across Desktop and Mobile.

---

## 🚀 Quick Start

### 1. Ignite the Forge
Install the [Dioxus CLI](https://dioxuslabs.com/learn/0.5/getting_started):
```bash
cargo install dioxus-cli

```

### 2. Choose Your Platform

Run your desired target directly from the root:

```bash
# Launch Desktop
dx serve --package desktop

# Launch Mobile
dx serve --package mobile

```

---

## 🎨 Development Workflow

Adding features follows the **Ignite Pattern**:

1. **Define Data** in `packages/api`.
2. **Craft UI** in `packages/ui` using atomic components and scoped styles.
3. **Deploy** to your platform packages.

```rust
// Example: Using the shared UI in your views
use ui::components::button::Button;

pub fn Home() -> Element {
    rsx! {
        Button { 
            label: "Ignited in Rust",
            onclick: |_| println!("System online!") 
        }
    }
}

```

---

## 🛠 Project Standards

* **Lints:** Strict workspace-wide linting via `clippy.toml`.
* **Theming:** Centralized CSS variables in `ui/src/assets/styling/`.
* **License:** MIT

---

**Built with ❤️ and 🦀 for the Rust community.**

```

**Would you like me to help you create a `cargo-generate.toml` so people can use `cargo generate --git dx-ignite-template` to start their projects with custom names automatically?**

```
