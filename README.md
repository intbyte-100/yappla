# yappla

About
-----
yappla is a small, fast GTK4 launcher for Linux written in Rust using Relm4. It provides a minimal, keyboard-driven UI for searching and launching items in different "modes" (applications, echoing stdin). The app is lightweight, supports theming using CSS, and uses fuzzy search to rank results.

Features
--------
- Multiple modes:
  - `apps` — discover and launch `.desktop` applications (parses XDG application directories)
  - `echo` — read lines from stdin and present them as selectable entries
- Fuzzy search using `strsim` (Jaro–Winkler and normalized Levenshtein)
- Simple keyboard navigation (arrow keys, Enter, Escape)
- Themable via CSS (`yappla.css`, or fallback bundled CSS)
- Planned: `json` mode — read structured JSON from stdin to present custom items (not implemented yet)
- Planned: mapping non-English symbols to English equivalents during search to improve matching for users with several keyboard layouts (not implemented yet)


Requirements
------------
- Rust toolchain 
- GTK4 development libraries available on your platform

Build
-----
Build release binary with Cargo:

```bash
cargo build --release -p yappla
```

Run examples
------------
- Run the `apps` mode (search installed desktop apps):

```bash
yappla apps
```

- Run the `echo` mode — feed lines on stdin and then search/select them:

```bash
printf "first\nsecond\nthird\n" | yappla echo
```

Theming / configuration
-----------------------
yappla tries to load CSS in this order:
1. `$HOME/.config/yappla/yappla.css`
2. `./yappla.css` (in the current working directory)
3. Built-in fallback `theme.css` included in the binary

A default theme file is included as `yappla/theme.css`. To override appearance, create the configuration file in your home config directory above or drop a `yappla.css` next to the binary when running.

Keyboard shortcuts
------------------
- Escape — exit
- Up / Down — move selection
- Enter — run the selected item
- Typing in the entry updates the search query

