# cuda-time

Time utilities — deadlines, timers, time windows, duration math, temporal context (Rust)

Part of the Cocapn temporal layer — deadlines, scheduling, and time reasoning.

## What It Does

### Key Types

- `Deadline` — core data structure
- `TimeWindow` — core data structure
- `Timer` — core data structure
- `TimeManager` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-time.git
cd cuda-time

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_time::*;

// See src/lib.rs for full API
// 11 unit tests included
```

### Available Implementations

- `Deadline` — see source for methods
- `TimeWindow` — see source for methods
- `Timer` — see source for methods
- `TimeManager` — see source for methods

## Testing

```bash
cargo test
```

11 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: time
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-timing](https://github.com/Lucineer/cuda-timing)
- [cuda-deadlock](https://github.com/Lucineer/cuda-deadlock)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
