# MyOS

MyOS is a personal Wayland desktop shell for Fedora. The first implementation runs as ordinary user processes above an existing compositor such as labwc or sway.

## Current slice

- Rust workspace with separate shell and state-engine processes.
- GTK4 top bar anchored through `gtk4-layer-shell`.
- Clock and launcher button on the top layer.
- Unix-socket heartbeat service at `$XDG_RUNTIME_DIR/myos-state.sock`.
- `systemd --user` units with independent restart behavior.

The compositor remains responsible for DRM/KMS, input, window management, and presentation. MyOS does not replace it.

## Fedora setup

```text
sudo dnf install cargo rust gtk4-devel gtk4-layer-shell-devel pkg-config
```

Use a Wayland session with labwc or sway. Build and run the development slice:

```text
cargo run -p myos-state-engine
cargo run -p myos-shell
```

The shell expects `WAYLAND_DISPLAY` to point to an active Wayland session. The launcher uses `$TERMINAL`, falling back to `foot`.

## User services

After building, edit the paths in `systemd/` if the checkout is not at `%h/myOS/MyOS`, then install them:

```text
mkdir -p ~/.config/systemd/user
cp systemd/myos-state-engine.service systemd/myos-shell.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now myos-state-engine.service myos-shell.service
```

## Next milestones

1. Replace the heartbeat with NetworkManager, UPower, and PipeWire D-Bus subscriptions.
2. Add a real launcher popover and a widget drawer.
3. Add ownership and lifecycle checks around the runtime socket.
4. Add nested Wayland smoke tests before session registration or packaging.
