# More Chat History (`more_chat_history`)

A native [Horizon](https://playhorizon.dev) mod that expands the client's chat history buffer and wrapped lines cache from the default limit of **100 lines** to **10,000 lines**, allowing players to scroll far up in chat to review older messages and logs.

## What it does

In vanilla Horizon client rendering (`crates/horizon-client-render/src/chat.rs`), chat history is truncated:
* `Chat::add_message`: `self.messages.truncate(100)`
* `Chat::add_lines`: `self.lines.truncate(100)`

This mod uses `horizon-mixins` with `MODIFY_CONSTANT` injections targeting both locations to replace the `100` literal with `10_000`, without touching or modifying any other game behavior or network packets.

## Repository Layout

```text
More-Chat-History/
├── more_chat_history-0.1.0.hnmod # Packaged distribution file
├── .gitignore
├── Cargo.toml                  # Standalone crate definition for local cargo check
├── LICENSE                     # MIT License
├── README.md                   # Mod overview and usage instructions
├── compatibility.json          # Pinning Horizon revision and source checksums
├── package.py                  # Packager script generating the deterministic .hnmod archive
└── mixins/
    └── more_chat_history/
        ├── horizon-mixin.toml  # Mixin manifest (injections & target definitions)
        ├── runtime.rs          # Mod logic & unit test module
        ├── limit_messages.rs   # Helper modifying the message buffer cap
        └── limit_lines.rs      # Helper modifying the wrapped lines cap
```

## Building & Testing

### 1. In-Game Local Test Build
Build and run directly using the Horizon launcher:
```bash
horizon-launcher native-build mixins/more_chat_history --trust-native-code
```

### 2. Repackaging the `.hnmod` File
```bash
python package.py
```
Outputs `more_chat_history-0.1.0.hnmod`.

## Submitting to the Registry

1. Download or use `more_chat_history-0.1.0.hnmod`.
2. Go to [playhorizon.dev/mods/submit](https://playhorizon.dev/mods/submit) and sign in with GitHub.
3. Select **Native Mod** and upload `more_chat_history-0.1.0.hnmod`.
