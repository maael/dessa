# Dessa

A basis to try to build a GW2 DLL to send Mumble Link and ArcDPS information via WebSockets.

This is Rust and I have no idea what I'm doing.

![I have no idea what I'm doing](docs/ihavenoidea.jpg)

## Run

```rs
cargo run
```

It will then expose a websocket server on `ws://localhost:3012`, which will emit `{"start": 1}` and then data every 1 second afterwards.

You can use [this](https://www.websocket.org/echo.html) to test it.

## TODOs

- [ ] Put into a GW2 DLL format - see [Blish HUD ArcDPS plugin](https://github.com/blish-hud/arcdps-bhud) for example.
- [ ] Get data from MumbleLink shared memory and send it in data.
- [ ] Get ArcDPS data the same way as [Blish HUD ArcDPS plugin](https://github.com/blish-hud/arcdps-bhud) does but send it as JSON.
