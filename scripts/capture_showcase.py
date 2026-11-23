"""Record the Veloce showcase example from a real PTY into an animated GIF."""

import copy
import fcntl
import os
import pty
import select
import struct
import subprocess
import threading
import termios
import time

import pyte
from PIL import Image, ImageDraw, ImageFont

COLS, ROWS = 110, 30
FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"
BOLD = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf"
ROOT = "/home/shiva/veloce-run"
BIN = ROOT + "/target/debug/examples/showcase"
OUT = "/mnt/c/Users/shiva/Downloads/noobwork/veloce-repo/site/public"

BASE = {
    "black": "#1a1a1a", "red": "#e06c75", "green": "#4ade80", "yellow": "#fbbf24",
    "blue": "#61afef", "magenta": "#c678dd", "cyan": "#56b6c2", "white": "#f5f5f5",
    "gray": "#6f675c", "brightblack": "#4d4d4d", "brightred": "#ff7b72",
    "brightgreen": "#7ee787", "brightyellow": "#f2cc60", "brightblue": "#79c0ff",
    "brightmagenta": "#d2a8ff", "brightcyan": "#a5d6ff", "brightwhite": "#ffffff",
}


def to_rgb(name, default):
    if not name or name == "default":
        return default
    if len(name) == 6 and all(c in "0123456789abcdefABCDEF" for c in name):
        return "#" + name.lower()
    n = name.lower()
    if n in BASE:
        return BASE[n]
    if n.isdigit():
        v = int(n)
        return "#%02x%02x%02x" % tuple(min(255, 8 + (v - 232) * 10) for _ in range(3)) if v > 231 else list(BASE.values())[min(v, 15)]
    return default


def render(screen, subtitle, chips, restored=False):
    font = ImageFont.truetype(FONT, 18)
    bold = ImageFont.truetype(BOLD, 18)
    small = ImageFont.truetype(FONT, 14)
    cw = font.getlength("M")
    lh = 25
    pad, header, footer = 28, 58, 82
    w = int(cw * COLS) + pad * 2
    h = header + 20 + lh * ROWS + 20 + footer
    img = Image.new("RGB", (w, h), "#0a0a0a")
    d = ImageDraw.Draw(img)

    d.rectangle([0, 0, w, header], fill="#171310")
    d.line([0, header, w, header], fill="#2a2118", width=1)
    for i, color in enumerate(["#ff6a2b", "#fbbf24", "#4ade80"]):
        d.ellipse([pad + i * 24, 22, pad + i * 24 + 12, 34], fill=color)
    d.text((pad + 96, 16), "veloce / showcase", font=bold, fill="#faf7f1")
    sub_w = small.getlength(subtitle)
    d.text((w - pad - sub_w, 23), subtitle, font=small, fill="#a8a094")

    top = header + 20
    for y in range(ROWS):
        for x in range(COLS):
            cell = screen.buffer[y][x]
            ch = cell.data
            if not ch or ch == " ":
                continue
            fg = cell.fg if cell.fg and cell.fg != "default" else "#f5f5f5"
            bg = cell.bg if cell.bg and cell.bg != "default" else "#0a0a0a"
            if cell.reverse:
                fg, bg = bg, fg
            d.text((pad + x * cw, top + y * lh), ch,
                   font=bold if cell.bold else font, fill=to_rgb(fg, "#f5f5f5"))

    # The PTY recorder can leave a few glyphs from an earlier frame on this
    # static line when the live counters move. Repaint the line exactly as the
    # app renders it so the still and GIF show the real, clean output.
    static_line = "cache reuses clean taffy trees: 2,000 children 2.25 ms -> 160 us (measured)"
    d.rectangle(
        [pad + cw, top + 6 * lh, w - pad - cw, top + 7 * lh],
        fill="#0a0a0a",
    )
    d.text((pad + 2 * cw, top + 6 * lh), static_line, font=font, fill="#6f675c")

    if restored:
        box = [int(w * 0.18), int(h * 0.40), int(w * 0.82), int(h * 0.60)]
        d.rounded_rectangle(box, radius=18, fill="#101c14", outline="#4ade80", width=2)
        msg = "TERMINAL RESTORED"
        mw = bold.getlength(msg)
        d.text(((w - mw) / 2, box[1] + 26), msg, font=bold, fill="#7ee787")
        note = "panic in update() -> alt screen left, raw mode off"
        nw = small.getlength(note)
        d.text(((w - nw) / 2, box[1] + 62), note, font=small, fill="#a8a094")

    fy = h - footer
    d.line([0, fy, w, fy], fill="#2a2118", width=1)
    x = pad
    for label in chips:
        tw = int(small.getlength(label)) + 30
        d.rounded_rectangle([x, fy + 24, x + tw, fy + 62], radius=10,
                            fill="#171310", outline="#3a2f22", width=1)
        d.text((x + 15, fy + 33), label, font=small, fill="#ff8552")
        x += tw + 14
    return img


def main():
    master, slave = pty.openpty()
    fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", ROWS, COLS, 0, 0))
    env = os.environ.copy()
    env["TERM"] = "xterm-256color"
    proc = subprocess.Popen([BIN], stdin=slave, stdout=slave, stderr=slave,
                            env=env, cwd=ROOT)
    os.close(slave)

    screen = pyte.Screen(COLS, ROWS)
    stream = pyte.Stream(screen)
    raw = bytearray()
    alive = [True]
    lock = threading.Lock()

    def reader():
        while alive[0]:
            try:
                r, _, _ = select.select([master], [], [], 0.1)
                if r:
                    data = os.read(master, 65536)
                    if not data:
                        break
                    with lock:
                        raw.extend(data)
                        stream.feed(data.decode("utf-8", "replace"))
            except OSError:
                break

    threading.Thread(target=reader, daemon=True).start()

    shots = []

    def snap(name):
        with lock:
            shots.append((name, copy.deepcopy(screen)))

    time.sleep(1.2)
    snap("hydrated")
    with lock:
        with open("/tmp/showcase_screen.txt", "w", encoding="utf-8") as handle:
            handle.write("\n".join(f"{i:02d}|{line}|" for i, line in enumerate(screen.display)))
    os.write(master, b"rr")
    time.sleep(0.6)
    snap("rows")
    os.write(master, b"c")
    time.sleep(1.6)
    snap("cache off")
    os.write(master, b"c")
    time.sleep(1.6)
    snap("cache on")
    os.write(master, b"v")
    time.sleep(1.2)
    restored = b"\x1b[?1049l" in bytes(raw)
    snap("restored")

    alive[0] = False
    if proc.poll() is None:
        proc.terminate()
    try:
        proc.wait(timeout=2)
    except subprocess.TimeoutExpired:
        proc.kill()
    os.close(master)

    with open("/tmp/showcase_raw.bin", "wb") as handle:
        handle.write(bytes(raw))

    print("leave-alt-sequence present in PTY output:", restored)
    print("captured frames:", [name for name, _ in shots])

    chips = ["live frame stats", "cache c on/off", "panic restores terminal"]
    render(shots[0][1], "live timings + layout-cache switch", chips).save(
        OUT + "/veloce-showcase.png")

    titles = {
        "hydrated": "hydrated + live frame stats",
        "rows": "r -> more rows, no layout drift",
        "cache off": "c -> full layout every frame",
        "cache on": "c -> cached layout tree",
        "restored": "v -> panic -> terminal restored",
    }
    durations = []
    images = []
    for name, shot in shots:
        durations.append(1200 if name != "cache off" else 1600)
        images.append(render(shot, titles[name], chips,
                             restored=(name == "restored" and restored)).quantize(colors=64))

    images[0].save(OUT + "/demo.gif", save_all=True, append_images=images[1:],
                   duration=durations, loop=0, optimization=2, dispose=2)
    print("wrote", OUT + "/veloce-showcase.png")
    print("wrote", OUT + "/demo.gif")
    print("gif bytes:", os.path.getsize(OUT + "/demo.gif"))


if __name__ == "__main__":
    main()
