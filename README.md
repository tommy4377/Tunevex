# Tunevex v0.3.0

Native Windows backend

Tunevex is a Windows system tuning and optimization utility built with Rust, Tauri and Svelte.

## Highlights

- More native registry, service and Windows API operations
- Stronger state detection and rollback semantics
- Safer device and system monitoring
- Large duplicate and bug cleanup

## Safety

Tunevex changes Windows system settings. Create a restore point before applying groups of tweaks and review higher-risk controls individually.

## Development

Run npm ci, npm run check and npm run tauri dev from the repository root.

## Release

Release tags publish one portable Windows executable named Tunevex.exe.

## License

MIT License. Maintained by @tommy4377.
