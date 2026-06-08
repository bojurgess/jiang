# jiang

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

## Runtime targets

The package ships three builds for different environments:

```ts
import { extractPalette } from "@bojurgess/jiang";          // bundler (Vite, webpack)
import { extractPalette } from "@bojurgess/jiang/web";      // browser ESM (no bundler)
import { extractPalette } from "@bojurgess/jiang/node";     // Node.js / Bun
```

## Building from source

Requires [wasm-pack](https://rustwasm.github.io/wasm-pack/).

```sh
make all       # builds bundler, web, and node targets
make publish   # builds then publishes to npm
```

## License

MIT