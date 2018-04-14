# hosts2unbound
Convert a hosts format block list to an unbound config fragment.

## Installation
0) Install Rust
0) `cargo build --release`
0) `scp target/release/hosts2unbound me@router:/usr/local/bin/`
0) `scp service/* me@router:/etc/systemd/system/`
0) `ssh me@server`
0) Add `include: /etc/unbound/auto-block-zones.conf` to the `server:` section of /etc/unbound/unbound.conf.
0) `sudo restorecon -Fv /etc/systemd/system/* /usr/local/bin/hosts2unbound`
0) `sudo systemctl daemon-reload`
0) `sudo systemctl enable update-unbound-hosts.timer`
