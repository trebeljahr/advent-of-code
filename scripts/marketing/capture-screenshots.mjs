// Renders the README screenshot.
//
//   node scripts/marketing/capture-screenshots.mjs
//
// 1. Builds the binary and runs it in a pseudo-terminal (python3 pty) to record
//    the real day-picker output, colour codes included.
// 2. Turns that output into HTML and renders it with headless Chromium to
//    docs/marketing/screenshots/day-picker.png.
//
// Playwright is not a project dependency: the script uses PLAYWRIGHT_MODULE, a
// local install, or the global one (`npm i -g playwright`), in that order.
// CHROMIUM_PATH may point at a specific Chromium build.

import { execFileSync, execSync } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const root = resolve(fileURLToPath(import.meta.url), "../../..");
const marketing = join(root, "docs/marketing");
mkdirSync(join(marketing, "screenshots"), { recursive: true });

async function loadPlaywright() {
  if (process.env.PLAYWRIGHT_MODULE) return createRequire(import.meta.url)(process.env.PLAYWRIGHT_MODULE);
  try {
    return await import("playwright");
  } catch {
    const globalRoot = execSync("npm root -g").toString().trim();
    return createRequire(join(globalRoot, "noop.js"))("playwright");
  }
}

// --- 1. Record the day picker -------------------------------------------------
execSync("cargo build --quiet", { cwd: root, stdio: "inherit" });
const targetDir = process.env.CARGO_TARGET_DIR ?? join(root, "target");
const binary = join(targetDir, "debug", "advent-of-code");

const recorder = `
import os, pty, select, struct, sys, time, fcntl, termios
pid, fd = pty.fork()
if pid == 0:
    os.environ["TERM"] = "xterm-256color"
    os.execv(sys.argv[1], [sys.argv[1]])
fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 80, 0, 0))
out, end = b"", time.time() + 2.5
while time.time() < end:
    ready, _, _ = select.select([fd], [], [], 0.2)
    if ready:
        try: out += os.read(fd, 4096)
        except OSError: break
os.write(fd, b"\\x1b")
time.sleep(0.3)
try: os.kill(pid, 9)
except OSError: pass
sys.stdout.buffer.write(out)
`;
const raw = execFileSync("python3", ["-c", recorder, binary], { cwd: root }).toString("utf8");
writeFileSync(join(marketing, "screenshots/day-picker.ansi"), raw);

// --- 2. ANSI to HTML -----------------------------------------------------------
// The picker only uses 256-colour foregrounds 10 (bright green) and 14 (bright
// cyan), resets and cursor movement. Everything else is stripped.
const palette = { 10: "#7ee787", 14: "#79c0ff" };
const escapeHtml = (s) => s.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
function ansiToHtml(text) {
  let html = "";
  let open = false;
  const parts = text.replace(/\r/g, "").split(/(\x1b\[[0-9;?]*[A-Za-z])/);
  for (const part of parts) {
    const sgr = part.match(/^\x1b\[([0-9;]*)m$/);
    if (sgr) {
      if (open) html += "</span>";
      open = false;
      const fg = sgr[1].match(/^38;5;(\d+)$/);
      if (fg) {
        html += `<span style="color:${palette[fg[1]] ?? "inherit"}">`;
        open = true;
      }
      continue;
    }
    if (part.startsWith("\x1b[")) continue;
    html += escapeHtml(part);
  }
  if (open) html += "</span>";
  return html.trimEnd();
}
const terminalHtml = ansiToHtml(raw);

// Card C shows the start of merge_overlapping_ranges in day 5, read from source.
const day5 = readFileSync(join(root, "src/2025/day_5/solution.rs"), "utf8").split("\n");
const start = day5.findIndex((line) => line.startsWith("fn merge_overlapping_ranges"));
const codeExcerpt = day5.slice(start, start + 18).join("\n");

const prepare = async (page) => {
  await page.evaluate(([t, c]) => {
    window.setTerminal(t);
    window.setCode(c);
  }, [terminalHtml, codeExcerpt]);
  await page.evaluate(() => document.fonts.ready);
};

// --- 3. Render -----------------------------------------------------------------
const { chromium } = await loadPlaywright();
const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_PATH || undefined });

const page = await browser.newPage({ viewport: { width: 1000, height: 400 }, deviceScaleFactor: 2 });
await page.goto(pathToFileURL(join(marketing, "social-preview/template.html")).href + "?v=terminal");
await prepare(page);
const frame = page.locator("#terminal-only");
await frame.screenshot({ path: join(marketing, "screenshots/day-picker.png") });
console.log("wrote docs/marketing/screenshots/day-picker.png");
await page.close();

await browser.close();
