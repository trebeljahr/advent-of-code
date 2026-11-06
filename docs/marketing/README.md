# Marketing assets

Screenshot and GitHub social preview cards for this repository. Every image is rendered from the real program output or the real source; nothing is drawn by hand. [`manifest.json`](manifest.json) lists sizes and SHA-256 hashes.

Regenerate everything with:

```sh
node scripts/marketing/capture-screenshots.mjs
```

The script needs `python3`, Rust and Playwright with Chromium (`npm i -g playwright && npx playwright install chromium`; set `PLAYWRIGHT_MODULE` or `CHROMIUM_PATH` to use a specific install).

| File | Purpose | Pixels | How it was made | Date |
| --- | --- | --- | --- | --- |
| [`screenshots/day-picker.png`](screenshots/day-picker.png) | README screenshot | 2000×764 | `cargo run` recorded in a pseudo-terminal ([`day-picker.ansi`](screenshots/day-picker.ansi)), rendered in a terminal frame from `social-preview/template.html?v=terminal` | 2026-11-06 |
| [`social-preview/social-preview-a.png`](social-preview/social-preview-a.png) | Social preview A, terminal-led | 1280×640 | `template.html?v=a` with the recorded menu | 2026-11-06 |
| [`social-preview/social-preview-b.png`](social-preview/social-preview-b.png) | Social preview B, typographic | 1280×640 | `template.html?v=b` | 2026-11-06 |
| [`social-preview/social-preview-c.png`](social-preview/social-preview-c.png) | Social preview C, dark, code-led | 1280×640 | `template.html?v=c` with the start of `merge_overlapping_ranges` from `src/2025/day_5/solution.rs` | 2026-11-06 |
| [`social-preview/template.html`](social-preview/template.html) | Source of the frame and the cards | – | HTML/CSS, system fonts (SF Pro, SF Mono / Menlo) | 2026-11-06 |

The social preview is uploaded by hand in the repository settings (Settings → Social preview). Text on the cards says "days 1 to 9"; update `template.html` when more days land.
