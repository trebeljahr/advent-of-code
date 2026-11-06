# Marketing assets

README screenshot for this repository. Every image is rendered from the real program output or the real source; nothing is drawn by hand. [`manifest.json`](manifest.json) lists sizes and SHA-256 hashes.

Regenerate everything with:

```sh
node scripts/marketing/capture-screenshots.mjs
```

The script needs `python3`, Rust and Playwright with Chromium (`npm i -g playwright && npx playwright install chromium`; set `PLAYWRIGHT_MODULE` or `CHROMIUM_PATH` to use a specific install).

| File | Purpose | Pixels | How it was made | Date |
| --- | --- | --- | --- | --- |
| [`screenshots/day-picker.png`](screenshots/day-picker.png) | README screenshot | 2000×764 | `cargo run` recorded in a pseudo-terminal ([`day-picker.ansi`](screenshots/day-picker.ansi)), rendered in a terminal frame from `social-preview/template.html?v=terminal` | 2026-11-06 |
| [`social-preview/template.html`](social-preview/template.html) | Source of the terminal frame | – | HTML/CSS, system fonts (SF Pro, SF Mono / Menlo) | 2026-11-06 |

