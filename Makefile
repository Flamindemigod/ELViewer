dev:
	cargo tauri dev

prerelease:
	cargo tauri dev --release

release: build-linux build-windows

build-linux:
	cargo tauri build

build-windows:
	cargo tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc
