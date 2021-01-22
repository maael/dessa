# Dessa

A basis to try to build a GW2 DLL to send Mumble Link and ArcDPS information via WebSockets.

This is Rust and I have no idea what I'm doing.

![I have no idea what I'm doing](docs/ihavenoidea.jpg)

You can use the [websocket echo tool](https://www.websocket.org/echo.html) to test it.

## Contributing

### Clone the repo

```powershell
git clone https://github.com/maael/dessa
```

# Build it

You need a somewhat recent `rust` version. I didn't check the minimum version. dessa is built against the latest `main` channel.

1. Install `rustc`. For example via [rustup](https://rustup.rs/).
2. Install `cargo`. This is not necessary if you used [rustup](https://rustup.rs/).
3. Build it:
```powershell
cargo build --release
```
4. Copy `target\release\dessa.dll` into your bin64 folder of Guild Wars 2:
5. Start Gw2

## TODOs

- [ ] Put into a GW2 DLL format - see [Blish HUD ArcDPS plugin](https://github.com/blish-hud/arcdps-bhud) for example.
- [ ] Get data from MumbleLink shared memory and send it in data.
- [ ] Get ArcDPS data the same way as [Blish HUD ArcDPS plugin](https://github.com/blish-hud/arcdps-bhud) does but send it as JSON.
