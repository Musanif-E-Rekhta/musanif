# Product

## Register

product

Visual mood is load-bearing here. Themes, typography, and pacing are part of what we ship, not decoration over the workflow. Surfaces should still be evaluated as product (the reader app, settings, shelf, admin pipeline), but with the awareness that atmosphere does as much work as affordance.

## Users

Three readers reach for Musanif, and no single one is primary:

- **The diaspora reader.** South Asian, lives abroad, comes to Urdu in evenings and on weekends. Comfort and approachability matter; the app is one of a few places where they engage with the language at length.
- **The native daily reader.** Reads in Pakistan or India, treats Urdu as primary. Expects the app to feel native and quick, not like a translated product. Performance, density, and Nastaliq typography are not negotiable.
- **The student or scholar.** Studying ghazals, novels, criticism. Wants depth: chapter-level navigation, references, annotations. Tolerates more density than the other two but is repelled by clutter.

All three share screens. The design has to flex without picking a favorite.

## Product Purpose

Musanif takes the depth of a Rekhta-class Urdu corpus and rebuilds it as a 2026-native product across web, desktop, mobile, and admin. The corpus model is the starting point, not the ceiling: catalog access alone is solved. The unsolved problem is making that catalog feel like a place a literate person wants to spend time, on whatever device they have, in whatever mood they're in.

Success looks like: the reader chooses Musanif over the alternatives because reading there is materially nicer, not because it's the only option. The admin side (Sarab pipeline) is in service of making the reader side worth opening.

## Brand Personality

**Warm. Considered. Native.**

- **Warm**: parchment is the default theme for a reason. Even in midnight or ink, the palette carries some heat; tone is never clinical. Hospitality before performance theater.
- **Considered**: few choices, each defended. Refuse the urge to add a setting, a card, or a chip because the kit had room for one. Subtraction is a feature.
- **Native**: Urdu typography is the protagonist, not a localized afterthought. Nastaliq sets the rhythm; the chrome adapts to it.

The tone is closer to a literary magazine than to a SaaS dashboard. Closer to Reeder or Mubi than to Goodreads or a generic ebook reader.

## Anti-references

What Musanif should explicitly **not** look like:

- **Rekhta dot org**: web-1.0 catalog density, everything-above-the-fold, busy hero strips, gray-on-gray secondary chrome. The corpus model is the inspiration; the UI is not.
- **Kindle, Apple Books, Google Play Books**: generic e-reader chrome, library-as-database visuals, white cards on gray. Anti-mood: a database UI for a literary product.
- **Goodreads, StoryGraph**: social-network-for-books framing. No follower counts, no review-noise feeds, no avatar-first chrome. Not a social network; a reading room.
- **Generic SaaS / AI dashboards**: hero-metric templates, blue gradients, "Discover insights with AI" patterns, card grids that all look the same. This is the category-reflex trap; refuse it even when the kit hands it to you.

## Design Principles

1. **Urdu reads first.** Any view that contains Nastaliq lets the script set the rhythm; the chrome adapts to it. RTL is per-element and treated as content, not a global mirror flag. If a layout breaks once Nastaliq lands in it, the layout is wrong, not the script.
2. **Hospitality before density.** Rekhta's depth, hosted like a librarian rather than dumped like a database. Discovery surfaces should introduce, not exhaustively list. The catalog is real but the visit feels chosen.
3. **Mood is part of the product, not chrome.** Themes (parchment, midnight, sepia-dark, ink) and typography (Inter, Source Serif 4, Nastaliq) carry product weight equal to layout. Resist the reflex to strip atmosphere in the name of "clean"; clean is not the goal.
4. **Three readers, one room.** Every screen has to work for the diaspora reader, the native daily reader, and the scholar. No one feels like the primary; no one feels like an afterthought. When in doubt, pick the choice that lets all three stay.
5. **Refuse the category reflex.** The first design that comes to mind for "Urdu reading app" is wrong by definition: it is either Kindle, Rekhta, or a SaaS dashboard. Treat the obvious answer as a starting point to reject. The Islands UI metaphor, the four themes, and the Slack/Zen/GitLab-rooted vocabulary exist to keep us out of those grooves.

## Accessibility & Inclusion

- **WCAG 2.2 AA across all four themes.** Contrast, focus-visible, full keyboard navigation. Ink (true-black OLED) and parchment (warm cream) are the contrast extremes; both must pass independently, not on average.
- **RTL Urdu as first-class content.** Nastaliq rendered with appropriate line-height and letter-spacing; bidi handled per element with `dir="rtl" lang="ur"`. No global RTL inversion of the chrome; the script flows inside content while the app shell stays consistent across users who don't read Urdu but still use Musanif (e.g. an admin reviewing a Sarab job).
- **Reduced motion respected throughout.** Any motion (rail collapse, theme transitions, tab strip animation, scroll effects) honors `prefers-reduced-motion: reduce`. Motion is never the only way state changes; it accompanies a visual change that works without it.
- **Low-light legibility is part of accessibility, not just mood.** Midnight, sepia-dark, and ink are not "dark modes for vibes." They serve users with light sensitivity, astigmatism, or evening reading habits, and are validated as such. Theme choice is a real input to the design, never a fallback toggle.
