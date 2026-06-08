# jiang
[![npm](https://img.shields.io/npm/v/@bojurgess/jiang)](https://www.npmjs.com/package/@bojurgess/jiang)
[![npm downloads](https://img.shields.io/npm/dm/@bojurgess/jiang)](https://www.npmjs.com/package/@bojurgess/jiang)
[![bundle size](https://img.shields.io/badge/wasm-106kb%20brotli-blue)](https://github.com/bojurgess/jiang)
> *"I'll give you a hint, his name's Yang! He won a national math competition in China! He doesn't even speak English! Yeah, I'm sure of the math."*
<p>
  <img src="https://media1.tenor.com/m/j_VrmPkKLwUAAAAC/quant-quantitative.gif" alt="The Big Short" />
</p>

A fast, runtime-agnostic colour quantization and palette extraction library for JavaScript, compiled from Rust to WebAssembly.

---

## Installation

```sh
npm install @bojurgess/jiang
```

## Usage

```ts
import { extractPalette } from "@bojurgess/jiang";

const response = await fetch("https://example.com/image.jpg");
const buf = await response.arrayBuffer();

const palette = await extractPalette(new Uint8Array(buf), 64);

console.log(palette.dominant?.hex);           // "#a3c2f1"
console.log(palette.dominant?.rgb);           // [163, 194, 241]
console.log(palette.dominant?.hsl);           // [213.6, 0.72, 0.79]
console.log(palette.dominant?.population);    // 14203
console.log(palette.dominant?.titleTextColor) // "#000000"
```

If you already have raw RGBA pixel data (e.g. from a canvas):

```ts
import { extractPaletteFromRgba } from "@bojurgess/jiang";

const ctx = canvas.getContext("2d");
const { data } = ctx.getImageData(0, 0, canvas.width, canvas.height);

const palette = extractPaletteFromRgba(data, 64);
```

## Palette

Each call returns a `Palette` object with up to five named swatches:

| Role | Description |
|---|---|
| `dominant` | The most representative colour overall |
| `accent` | A vivid, saturated colour suitable for highlights |
| `subtle` | A muted, low-saturation colour |
| `dark` | The darkest usable colour |
| `light` | The lightest usable colour |

Each swatch (if present) has:

| Field | Type | Description |
|---|---|---|
| `hex` | `string` | e.g. `"#a3c2f1"` |
| `rgb` | `[number, number, number]` | e.g. `[163, 194, 241]` |
| `hsl` | `[number, number, number]` | Hue (0–360), saturation, lightness (0–1) |
| `population` | `number` | How many pixels fell into this colour's bucket |
| `titleTextColor` | `string` | `"#ffffff"` or `"#000000"` — WCAG AA contrast (4.5:1) |
| `bodyTextColor` | `string` | `"#ffffff"` or `"#000000"` — large text contrast (3.0:1) |

## Supported formats

`extractPalette` accepts raw bytes for the following formats:

| Format | Notes |
|---|---|
| JPEG | Decoded via `zune-jpeg` |
| PNG | Decoded via `minipng`, all colour modes and bit depths supported |
| WebP | Decoded via `image-webp`, both lossy and lossless |

Format is detected automatically from magic bytes, no file extension or mime type needed.

If you already have decoded RGBA pixel data, use `extractPaletteFromRgba` instead to skip decoding entirely.

## Runtime targets

The package ships three builds for different environments:

```ts
import { extractPalette } from "@bojurgess/jiang";          // bundler (Vite, webpack)
import { extractPalette } from "@bojurgess/jiang/web";      // browser ESM (no bundler)
import { extractPalette } from "@bojurgess/jiang/node";     // Node.js / Bun
```

The wasm binary is identical across all three, only the JS glue differs in how the module is instantiated. This means the library works anywhere JavaScript runs: browsers, Node, Deno, Bun, Cloudflare Workers, and other edge runtimes. It has no dependency on the DOM, `canvas`, or any browser-specific API.

This also makes it compatible with environments where canvas access is unavailable or restricted, including privacy-hardened browsers (Tor Browser, Firefox with `resistFingerprinting`) where `getImageData` is blocked as a fingerprinting vector.

## Bundle size

The library is designed to intentionally be lean, making use of decode-only Rust crates for supported formats. The published wasm binary is ~300kb uncompressed, ~106kb brotli.

## Building from source

Requires [wasm-pack](https://rustwasm.github.io/wasm-pack/) and [wasm-opt](https://github.com/WebAssembly/binaryen) (from Binaryen).

On Arch Linux:
```sh
sudo pacman -S binaryen
```

On macOS:
```sh
brew install binaryen
```

Then:
```sh
make all       # builds bundler, web, and node targets with wasm-opt applied
make publish   # builds then publishes to npm
```

`wasm-opt` is run automatically as part of `make all` with `-Oz` to reduce binary size.

## License

MIT