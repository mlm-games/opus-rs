# opus-rs

A pure-Rust implementation of the [Opus audio codec](https://opus-codec.org/) (RFC 6716), ported from the reference C implementation (libopus 1.6).

> **Status: Production-ready** — SILK-only, CELT-only, and Hybrid modes are functional. Stereo encoding (SILK and CELT) is supported.

## Features

- **Pure Rust** — no C dependencies, no unsafe code in the codec core
- **SILK encoder & decoder** — narrowband (8 kHz), mediumband (12 kHz), wideband (16 kHz)
- **CELT encoder & decoder** — fullband (48 kHz) with MDCT, PVQ, energy quantization
- **Hybrid mode** — SILK for low frequencies + CELT for high frequencies
- **Range coder** — entropy coding with ICDF tables and Laplace distribution
- **VAD** — voice activity detection
- **HP filter** — variable-cutoff high-pass filter for VOIP mode
- **CBR / VBR** — both constant and variable bitrate modes
- **LBRR** — in-band forward error correction
- **Resampler** — high-quality resampling (up2, up2_hq)
- **Stereo** — mid-side encoding for both SILK and CELT


## Quick Start

```rust
use opus_rs::{OpusEncoder, OpusDecoder, Application};

// Encode
let mut encoder = OpusEncoder::new(16000, 1, Application::Voip).unwrap();
encoder.bitrate_bps = 16000;
encoder.use_cbr = true;

let input = vec![0.0f32; 320]; // 20ms frame at 16kHz
let mut output = vec![0u8; 256];
let bytes = encoder.encode(&input, 320, &mut output).unwrap();

// Decode
let mut decoder = OpusDecoder::new(16000, 1).unwrap();
let mut pcm = vec![0.0f32; 320];
let samples = decoder.decode(&output[..bytes], 320, &mut pcm).unwrap();
```

## Testing

```bash
cargo test
```

All 170+ tests pass, covering MDCT identity, PVQ consistency, SILK/CELT/Hybrid encode/decode roundtrip, resampler tests, and more.

### WAV Roundtrip

```bash
# Rust encoder/decoder
cargo run --example wav_test
```

### Stereo Tests

```bash
cargo run --example stereo_test
```

## Performance

### vs C Opus (libopus 1.6.1) on Apple Silicon

Latest measurements on Apple Silicon M-series (aarch64), compiled with `--release` (opt-level=3 + ThinLTO), criterion bench with 20 samples. All numbers are per-frame (mono).

#### Real Audio Roundtrip (902/1804 frames of real speech, encode + decode)

| Config | Pure Rust | C Opus | Ratio |
|--------|-----------|--------|-------|
| 8 kHz / 20 ms VoIP | **34.77 ms** | 36.31 ms | 0.96× (**Rust 4% faster**) |
| 16 kHz / 20 ms VoIP | **58.23 ms** | 59.37 ms | 0.98× (**Rust 2% faster**) |
| 16 kHz / 10 ms VoIP | 63.44 ms | **62.50 ms** | 1.02× (C 2% faster) |
| 48 kHz / 20 ms Audio | **29.61 ms** | 32.42 ms | 0.91× (**Rust 9% faster**) |
| 48 kHz / 10 ms Audio | 34.58 ms | **33.47 ms** | 1.03× (C 3% faster) |

**Summary:**
- **Real audio 48 kHz**: Rust is **9% faster** on 20ms, within 3% on 10ms
- **Real audio SILK/VoIP**: Rust is 2–4% faster on 8/16 kHz 20ms, within 2% on 10ms

## License

See [COPYING](COPYING) for the original Opus license (BSD-3-Clause).

## Links

- **RustPBX**: <https://github.com/restsend/rustpbx>
- **RustRTC**: <https://github.com/restsend/rustrtc>
- **SIP Stack**: <https://github.com/restsend/rsipstack>
- **Rust Voice Agent**: <https://github.com/restsend/active-call>
