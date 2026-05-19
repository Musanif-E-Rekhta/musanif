# CLAUDE.md

Project notes for agent sessions. Keep terse; PRODUCT.md and DESIGN.md own the long form.

## Design Context

Read [PRODUCT.md](PRODUCT.md) before any UI work. The short version:

- **Register**: product, but visual mood is load-bearing (themes and typography are part of what we ship, not chrome).
- **Personality**: warm, considered, native. Closer to a literary magazine than a SaaS dashboard.
- **Anti-references**: Rekhta dot org, Kindle / Apple Books, Goodreads, generic SaaS / AI dashboards. The category reflex is the trap; refuse it.
- **Three readers, one room**: diaspora reader, native daily reader, and student / scholar all share screens; no primary.
- **A11y**: WCAG 2.2 AA across all four themes (parchment, midnight, sepia-dark, ink); RTL Urdu is first-class content, not a global mirror; `prefers-reduced-motion` honored everywhere.

[DESIGN.md](DESIGN.md) (when present) holds tokens, components, motion, and the visual system.
