<h2 align="center">Corrosive Rust Web App</h2>

Let's see what's possible with the leptos wasm machine...

### Notes

See [Trunk.toml](/Trunk.toml) for build configuration

**Trunk Commands**

```ps1
# Setup CLI 
cargo install trunk

# Run
trunk serve --release

# Build Dist.
trunk build --release

```

**VS Code Rust Analyser Setting**

Rust analyser doesn't approve of leptos macros without the following settings :

```json
 "rust-analyzer.procMacro.ignored": {
        "leptos_macro": [
            // optional:
            "component",
            "server"
        ],
    },

```