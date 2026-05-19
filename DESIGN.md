---
name: Musanif
description: Islands UI for an Urdu reading platform, four themes carrying mood as a product input.
colors:
  ledger-terracotta: "#b8412f"
  ledger-terracotta-hover: "#9c3525"
  ledger-terracotta-soft: "#1eb8412f"
  aged-parchment: "#f4ede1"
  fresh-folio: "#fffdf9"
  linen-ledger: "#ede4d2"
  manuscript-ink: "#221d1c"
  charcoal-bisque: "#6a5d59"
  faded-note: "#998a85"
  old-rose: "#e0adb0"
  old-rose-light: "#f1d6d8"
  aged-margin: "#e3d5be"
  gilt-highlight-bg: "#fef3c7"
  gilt-highlight-text: "#92400e"
  closed-book-midnight: "#0f1216"
  midnight-card: "#181c22"
  midnight-subtle: "#1f242c"
  midnight-text: "#ebe9e4"
  midnight-muted: "#98a0a8"
  banked-coal-salmon: "#ec7666"
  banked-coal-hover: "#f48b7c"
  midnight-margin: "#262c34"
  worn-mahogany: "#1c1610"
  mahogany-card: "#261d16"
  mahogany-subtle: "#322619"
  mahogany-text: "#efe5d6"
  mahogany-muted: "#ad9c8d"
  hearth-salmon: "#ec8775"
  inkpot-black: "#000000"
  inkpot-card: "#0c0c0d"
  inkpot-subtle: "#161616"
  inkpot-text: "#f0eeea"
  oled-salmon: "#ed7561"
typography:
  display:
    fontFamily: "Source Serif 4, Georgia, serif"
    fontSize: "32px"
    fontWeight: 600
    lineHeight: 1.15
    letterSpacing: "-0.02em"
  headline:
    fontFamily: "Inter, system-ui, sans-serif"
    fontSize: "24px"
    fontWeight: 800
    lineHeight: 1.15
    letterSpacing: "-0.01em"
  title:
    fontFamily: "Inter, system-ui, sans-serif"
    fontSize: "16px"
    fontWeight: 700
    lineHeight: 1.3
    letterSpacing: "-0.005em"
  body:
    fontFamily: "Inter, system-ui, sans-serif"
    fontSize: "13.5px"
    fontWeight: 400
    lineHeight: 1.5
    letterSpacing: "normal"
  reader-body:
    fontFamily: "Source Serif 4, Georgia, serif"
    fontSize: "17px"
    fontWeight: 400
    lineHeight: 1.75
    letterSpacing: "normal"
  label:
    fontFamily: "Inter, system-ui, sans-serif"
    fontSize: "10.5px"
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: "0.10em"
  urdu:
    fontFamily: "Noto Nastaliq Urdu, Jameel Noori Nastaleeq, serif"
    fontSize: "22px"
    fontWeight: 700
    lineHeight: 1.4
    letterSpacing: "normal"
rounded:
  xs: "4px"
  sm: "6px"
  md: "8px"
  lg: "10px"
  xl: "12px"
  xxl: "14px"
  full: "9999px"
spacing:
  xxs: "4px"
  xs: "6px"
  sm: "8px"
  md: "12px"
  lg: "14px"
  xl: "18px"
  xxl: "24px"
  xxxl: "32px"
components:
  button-default:
    backgroundColor: "{colors.fresh-folio}"
    textColor: "{colors.manuscript-ink}"
    typography: "{typography.body}"
    rounded: "{rounded.sm}"
    padding: "7px 12px"
  button-primary:
    backgroundColor: "{colors.ledger-terracotta}"
    textColor: "{colors.fresh-folio}"
    typography: "{typography.body}"
    rounded: "{rounded.sm}"
    padding: "7px 12px"
  button-primary-hover:
    backgroundColor: "{colors.ledger-terracotta-hover}"
    textColor: "{colors.fresh-folio}"
  button-ghost:
    backgroundColor: "transparent"
    textColor: "{colors.charcoal-bisque}"
    typography: "{typography.body}"
    rounded: "{rounded.sm}"
    padding: "7px 12px"
  chip-default:
    backgroundColor: "{colors.linen-ledger}"
    textColor: "{colors.charcoal-bisque}"
    rounded: "{rounded.sm}"
    padding: "2px 8px"
  chip-primary:
    backgroundColor: "{colors.ledger-terracotta-soft}"
    textColor: "{colors.ledger-terracotta}"
    rounded: "{rounded.sm}"
    padding: "2px 8px"
  chip-star:
    backgroundColor: "{colors.gilt-highlight-bg}"
    textColor: "{colors.gilt-highlight-text}"
    rounded: "{rounded.sm}"
    padding: "2px 7px"
  input-search:
    backgroundColor: "{colors.linen-ledger}"
    textColor: "{colors.manuscript-ink}"
    typography: "{typography.body}"
    rounded: "{rounded.md}"
    padding: "6px 11px"
  island:
    backgroundColor: "{colors.fresh-folio}"
    textColor: "{colors.manuscript-ink}"
    rounded: "{rounded.xxl}"
  island-soft:
    backgroundColor: "{colors.linen-ledger}"
    textColor: "{colors.manuscript-ink}"
    rounded: "{rounded.xxl}"
  nav-item:
    backgroundColor: "transparent"
    textColor: "{colors.charcoal-bisque}"
    rounded: "{rounded.sm}"
    padding: "7px 10px"
  nav-item-active:
    backgroundColor: "{colors.ledger-terracotta-soft}"
    textColor: "{colors.ledger-terracotta}"
    rounded: "{rounded.sm}"
    padding: "7px 10px"
  tab:
    backgroundColor: "transparent"
    textColor: "{colors.charcoal-bisque}"
    typography: "{typography.body}"
    padding: "9px 14px"
  tab-active:
    backgroundColor: "transparent"
    textColor: "{colors.ledger-terracotta}"
    typography: "{typography.body}"
    padding: "9px 14px"
---

# Design System: Musanif

## 1. Overview: The Floating Library

**Creative North Star: "The Floating Library"**

Musanif's interface is a floating library: rounded "Islands" of card surface, drifting on a tinted page, holding an Urdu literary corpus. Navigation feels less like moving between web pages and more like moving between rooms in a calm building. The Zen-Browser influence is direct (generous radii, floating islands, soft warm shadows). The Slack influence is the density and compact grouping inside each island (Browse · Library · Collections with count pills, not a sprawling sidebar). The GitLab influence is the workmanlike header + actions + breadcrumb pattern that holds the admin pipeline together. Three references, none dominant; the synthesis is the system.

Four themes carry the mood, and the mood is part of the product, not a setting tucked away in chrome: **parchment** is daytime light, warm and unhurried; **midnight** is the calm of an evening session under lamplight; **sepia-dark** is the worn-manuscript register for low-light reading without OLED intensity; **ink** is for the AMOLED user who reads at 2am with the bedside lamp off. All four pass independent contrast checks; none of them are "vibes." Typography is the protagonist: Nastaliq (Noto Nastaliq Urdu) on cover art and the brand mark, Source Serif 4 for long-form reading and reader headlines, Inter everywhere else. The chrome adapts to the script, never the other way around.

This system explicitly rejects: web-1.0 catalog density (Rekhta dot org), database-as-library e-reader chrome (Kindle, Apple Books), social-network-for-books framing (Goodreads, StoryGraph), and SaaS-dashboard tropes (card-grid sameness, hero metric templates, AI-tool gradients). When the obvious answer arrives, treat it as the starting point to reject.

**Key Characteristics:**
- **Islands over panels.** Every major region of the screen is a floating, rounded surface (`border-radius: 14px`) with subtle warm shadow. The page is never edge-to-edge; the tint of the page-color shows between islands.
- **Mood is structural.** Four themes carry product weight; theme choice is an input to the design, not a fallback toggle.
- **Compact density, generous radii.** Type scale is tight (10–17px range for chrome; 22–32px for headlines and reader). Spacing is also tight (gaps in the 6–18px range). The breathing room comes from corner radius and tinted-page gutters, not padding.
- **Urdu as protagonist.** Nastaliq sets the rhythm in any view it appears in. The covers, the brand mark, and the author hero all use it; the chrome never fights it.
- **Restrained motion.** State changes only (100–150ms ease). No choreography, no scroll-driven sequences, no bounce.

## 2. Colors: The Four-Theme Palette

Four themes (parchment, midnight, sepia-dark, ink) share a token shape: each has `bg-color`, `bg-card`, `bg-subtle`, `text-main`, `text-muted`, `text-faint`, `primary`, `primary-hover`, `primary-soft`, `accent`, `accent-light`, `border-light`, `border-faint`, plus warm-tinted shadows. The primary is always a warm red-orange that shifts saturation and lightness across themes, never a different hue. The neutrals tint toward the primary (warm-cream, warm-gray, warm-mahogany) rather than to cool blue. There is no `#fff` and the only `#000` is in the ink theme, deliberately, because OLED panels reward true black.

### Primary

- **Ledger Terracotta** (`#b8412f`, parchment theme): the active state across the whole reader app. Used on links, the brand mark, primary buttons, the active-nav background tint, the drop cap on reader body, the progress fill, the rating accent, and the book-cover title in Nastaliq. The intensity is calibrated for cream paper, not cool gray.
- **Banked-Coal Salmon** (`#ec7666`, midnight theme): the same role under lamplight. Lighter and more chromatic to compensate for the dark surround; reads as a coal that's been warmed in the dark, not a hot accent.
- **Hearth Salmon** (`#ec8775`, sepia-dark theme): the warmer dark variant. Slightly more orange, slightly less red, to live alongside mahogany backgrounds.
- **OLED Salmon** (`#ed7561`, ink theme): the OLED variant. Tuned for true-black surrounds where high-chroma colors can vibrate; pulled fractionally cooler.

### Secondary

There is **no Secondary accent**. Musanif does not paint with a second hue. The "secondary" register is occupied by the **gilt** (warm yellow) used only for star ratings: `#fef3c7` on `#92400e` in parchment, with darker amber backgrounds and a cream label in the dark themes. Gilt is semantic, not decorative, it appears only where a rating, a starred annotation, or a highlight is being communicated.

### Neutral

Each theme has three surface tones and three text tones; they are not interchangeable.

- **Aged Parchment** (`#f4ede1`, page bg): the warm cream of an old book page in daylight. The default background for the entire app on the parchment theme. The tint between islands.
- **Fresh Folio** (`#fffdf9`, card bg): a hair off pure white, warmed toward the brand. Islands rest on this. **Never use `#fff`**, the contrast against Aged Parchment must read as warm-on-warmer, not white-on-cream.
- **Linen Ledger** (`#ede4d2`, subtle bg): the third tone. Used for soft islands (`.island--soft`), hover backgrounds on row cards, search input fill, and chip backgrounds. The intermediate paper.
- **Manuscript Ink** (`#221d1c`, text-main): the warm near-black for reading text. Tinted toward the primary; never pure black.
- **Charcoal Bisque** (`#6a5d59`, text-muted): secondary text and inactive nav items. Sits between text-main and text-faint, used liberally for metadata, eyebrows, and supporting copy.
- **Faded Note** (`#998a85`, text-faint): the third tier, used for eyebrows (uppercase 10px labels), inactive icons, and placeholder text. Quietest legible tone.
- **Closed-Book Midnight** (`#0f1216`, page bg): the midnight-theme equivalent of Aged Parchment. A deep, near-blue-black that reads as a closed book in a dim room. Not "GitHub dark"; warmer.
- **Worn Mahogany** (`#1c1610`, page bg): the sepia-dark page. Walnut/mahogany without the saturation of a dedicated sepia theme; for users who want low-light without OLED intensity.
- **Inkpot Black** (`#000000`, page bg): the only pure black in the system. OLED only. Card and subtle surfaces (`#0c0c0d`, `#161616`) immediately lift off black so the eye has structure.

### Named Rules

**The Warm Neutrals Rule.** Every neutral, in every theme, tints toward the brand's red-orange hue. Cool neutrals are prohibited. If a color reads "tech gray" or "GitHub gray," it is wrong and must be retoned warmer. The reading room is never lit by fluorescents.

**The One-Hue Primary Rule.** Across all four themes, the primary is a warm red-orange. Themes vary lightness and chroma; they never change the hue. If a future theme needs a new accent role, it adds a semantic token (e.g. gilt for ratings), not a second brand color.

**The No-Pure-White Rule.** `#fff` is prohibited on the parchment theme. The brightest surface is Fresh Folio (`#fffdf9`); pure white reads as a foreign body on warm cream. This rule does not apply to the dark themes' text-main, which are intentionally warm off-whites (`#ebe9e4`, `#efe5d6`, `#f0eeea`).

**The Gilt-Is-Semantic Rule.** Warm yellow (`#fef3c7` / `#92400e` in parchment) is used **only** for star ratings, starred annotations, and highlights. It is not a brand color. It does not appear in nav, buttons, or chrome.

## 3. Typography

**Display & Reader Font:** Source Serif 4 (Georgia fallback). Used for `.is-reader-h1`, `.is-reader-body` (long-form reading), and the stat-value font on profile and book-detail pages. The italic cut handles ledes.
**Body Font:** Inter (system-ui fallback). Used for everything chrome: nav, buttons, chips, page headers, tabs.
**Urdu Font:** Noto Nastaliq Urdu (Jameel Noori Nastaleeq fallback). Used for cover art titles, the brand mark in the navbar (the literal `م` and `Musanif` lockup), the author avatar glyph, the book-cover stamp, and any in-content Urdu.
**Mono:** none. Code rendering inside reader content falls back to Fira Code / Courier; there is no system-wide mono token.

**Character.** The pairing is a literary-magazine pairing: serif for the text the user is actually reading; sans for the UI that gets out of the way. Inter is warm enough at 13–14px not to fight the parchment surround; Source Serif 4 at 17px / 1.75 line-height is the long-form spec. Nastaliq is the protagonist whenever an Urdu glyph is present and is treated as content, not as a localization detail.

### Hierarchy

- **Display** (Source Serif 4, 600 / 32px / lh 1.15 / ls -0.02em): the reader chapter title (`.is-reader-h1`) and the book-detail page H1 (`.is-detail-h1`, 28px). Always serif; never used for chrome.
- **Headline** (Inter, 800 / 22–24px / lh 1.15 / ls -0.01em): the page header on Discover, Authors, Profile, Shelf. Heavy weight; tight tracking. Marks "you are on this page" without an eyebrow.
- **Title** (Inter, 700 / 16px / lh 1.3 / ls -0.005em): island headers (`.is-main-title`), section heads, the feature card title (which scales up to 22px / 800). Used where a region needs labeling but not announcing.
- **Body** (Inter, 400–500 / 13–14px / lh 1.5): the entire chrome reads at this size. Nav items, chips, buttons, book metadata, row cards. 65–75ch cap does not apply here; this is UI, not prose.
- **Reader Body** (Source Serif 4, 400 / 17px / lh 1.75): the long-form reading register. Max width 720px on chapter pages. Includes a serif drop cap on the first paragraph (`first-letter`, 3.4em, in Ledger Terracotta) as the chapter's opening gesture.
- **Label** (Inter, 700 / 10–11px / ls 0.10em / uppercase): eyebrows, stat labels, section eyebrows, meta labels, nav section dividers. Heavily tracked uppercase, used sparingly; the system relies on it to signal "this is metadata about the thing below," not "this is the thing."
- **Urdu** (Noto Nastaliq Urdu, 700 / 18–28px / lh 1.4): a discrete second hierarchy. The brand mark and author avatars use 20–24px; book-cover titles use 18px; the reader's in-content Urdu uses the reader-body size (17px) at 700 weight. Always `dir="rtl" lang="ur"`.

### Named Rules

**The Serif-For-Reading Rule.** Source Serif 4 is reserved for content the user is actually reading: chapter body, reader title, lede, book hero H1, blockquote, drop cap, and the stat numerals on profile (because numerals in a serif read as artifacts, not gauges). It is never used in nav, buttons, or chips. Crossing the boundary undermines the literary-vs-chrome distinction.

**The Nastaliq-Sets-Rhythm Rule.** In any view containing Nastaliq, the Nastaliq sets the line-height baseline; the chrome wraps around it. Nastaliq needs more vertical breathing room than Inter at the same x-height; 1.4 line-height is the floor, not the ceiling. If a layout breaks once Nastaliq lands in it, the layout is wrong, not the script.

**The Eyebrow-Or-Header, Not Both Rule.** A region gets either an uppercase tracked eyebrow OR a sentence-case title, never both stacked. Stacking the two is a SaaS template cliché; pick the one that earns its place.

## 4. Elevation

Musanif is **layered**, not flat. Shadows exist and they have meaning: every shadow is warm-tinted toward the brand hue (`rgba(40, 30, 20, ...)` in parchment, deeper blacks in the dark themes), and every shadow has a named role. There are no pure-black shadows on the parchment theme, the warm tint is what keeps the cards from feeling like a SaaS dashboard. The islands rest on the page, and the book covers lift further off the islands.

### Shadow Vocabulary

- **shadow-card** (`0 1px 2px rgba(40,30,20,.04), 0 4px 16px rgba(40,30,20,.05)`): the resting elevation for every island. Subtle enough that the warm border-faint does most of the structural work; the shadow is presence, not depth.
- **shadow-card-hover** (`0 2px 4px rgba(40,30,20,.05), 0 8px 24px rgba(40,30,20,.08)`): the lifted state for interactive book cards on hover. Always paired with `transform: translateY(-2px)` to give physical correspondence.
- **shadow-cover** (`0 2px 6px rgba(40,30,20,.10), 0 6px 18px rgba(40,30,20,.10)`): the book-cover shadow. Stronger than card; covers are the protagonist of the catalog and read as physical objects.
- **shadow-thumb** (`0 1px 3px rgba(0,0,0,.10)`): the small thumbnail shadow used on row cards (shelf, continue rail). A whisper, just enough to lift the 56px cover off the row.
- **shadow-pop** (`0 8px 28px rgba(40,30,20,.16)`): the popover/dropdown shadow. Used sparingly; if a popover is appearing, the user expects it to feel separate from the page, not floating above it.

### Named Rules

**The Warm-Shadow Rule.** Shadows on the parchment theme are tinted with `rgba(40, 30, 20, ...)`, not `rgba(0, 0, 0, ...)`. The warm tint is what keeps the system out of SaaS-dashboard territory. The same logic applies in reverse on the dark themes (shadows there can use straight rgba(0,0,0) because the surround is already warm-dark).

**The Cover-Lifts-Highest Rule.** In any given view, the elevation order from lowest to highest is: page tint → island → row-card thumbnail → book cover (display size) → popover. Covers are the protagonist of the catalog; they lift further than the chrome around them.

**The No-Decorative-Depth Rule.** Shadows respond to state (hover, focus, popover) or denote a real layer (covers lifting off islands). They are never applied for ambient depth or "modern feel." A flat card with no state has no shadow beyond the resting `shadow-card`.

## 5. Components

For each component, lead with the character line, then specify shape, color, states.

### Buttons

Quiet, library-grade. Buttons are 7px vertical padding, 12px horizontal, 7px radius, 13px body type. They are never the loudest thing on the screen.

- **Shape:** `7px` radius (`rounded.sm`), `7px 12px` padding. Inline-flex, 6px icon gap. No uppercase, no letter-spacing.
- **Default** (`.is-btn`): `Fresh Folio` background, `Aged Margin` border (`#e3d5be`), `Manuscript Ink` text. Hover deepens to `--hover-tint` (a 4% black overlay on parchment, 5% white on dark themes).
- **Primary** (`.is-btn--primary`): `Ledger Terracotta` background, no border (border-color: primary), `#fff` text, weight 600. Hover shifts to `Ledger Terracotta Hover` (`#9c3525`). The only place pure white is acceptable, against the saturated primary it reads as ink-on-stamp, not foreign.
- **Ghost** (`.is-btn--ghost`): transparent background, no border, muted text. Used for tertiary actions inside an island that already has structure.
- **Danger** (`.is-btn--danger`): primary text color, primary border at 35% opacity (`color-mix`). Hover fills with `primary-soft`. There is **no separate red** for danger; the system reuses the primary because the brand color is already a warm red and a second red would create competing accents.
- **Block** (`.is-btn--block`): full-width variant. Used inside auth forms and admin actions.

### Chips

Compact, typographically defined. Chips are 11px, 600 weight, 2px / 8px padding, 6px radius. They are content, not decoration.

- **Default** (`.is-chip`): `Linen Ledger` background, `Charcoal Bisque` text, `border-faint` border. Used for tag pills, genre labels.
- **Primary** (`.is-chip--primary`): `Ledger Terracotta Soft` (10% opacity) background, `Ledger Terracotta` text, no border. Used for emphasis chips (e.g. "Featured," "New").
- **Star** (`.is-chip--star`): `Gilt Highlight` (warm yellow). The semantic warm-yellow that's reserved for ratings.

### Cards and Containers

Islands are the foundation. A card is an island; an island is a card. The vocabulary is one primitive with three opacity variants.

- **Island** (`.island`): `Fresh Folio` background, `border-faint` 1px border, 14px radius, `shadow-card`, `overflow: hidden`. The structural unit; nav, main, rail, modals all sit inside islands.
- **Soft island** (`.island--soft`): `Linen Ledger` background. Used for sub-regions inside a main island (e.g. the feature card on Discover, the author hero, the profile hero).
- **Quiet island** (`.island--quiet`): transparent, no shadow, no border. Used where the island metaphor is structural but visually absent (e.g. content blocks inside a parent island).
- **Row card** (`.row-card`): horizontal layout with a 56px cover thumb. `bg-subtle` rest, `bg-card` on hover (subtle invert), `border-light` on hover. Used on shelf, authors list, search results.
- **Book card** (`.is-book`): 4:5 aspect cover with a warm placeholder, Urdu title in primary, author label below. Hover lifts the card 2px and deepens the cover's shadow.

### Inputs

Compact, framed by a border that reveals on focus. The default state is integrated, not floating.

- **Search** (`.is-search`): `Linen Ledger` background, transparent 1px border that becomes `Ledger Terracotta` on focus (which also shifts the background to `Fresh Folio`). 8px radius, 6px / 11px padding. The kbd hint (`⌘K`) sits inline at the right, in a 4px-radius pill with a 10px body.
- Form inputs (auth, settings) follow the same logic at slightly larger sizes; the design intent is "the field is a slot, focus reveals it."

### Navigation

The nav is a Slack-style sidebar with named sections.

- **Nav island** (`.is-nav`): 240px column, flex-vertical, 14px / 10px padding. Brand mark at top (Urdu glyph + wordmark with bottom border). Sections labeled with 10px uppercase eyebrows; items at 13.5px body, 500 weight.
- **Nav item rest** (`.is-nav-item`): transparent background, `text-muted`, `hover-tint` background on hover.
- **Nav item active** (`.is-nav-item--active`): `primary-soft` background, `Ledger Terracotta` text, weight 600. The existing CSS also includes a 3px-wide left pseudo stripe; see Don'ts, this is being phased out in favor of the background tint alone.
- **Nav count pill** (`.is-nav-item-count`): a soft pill on the right, `bg-subtle` background, 10.5px tabular body. Used for unread or item counts.
- **Rail nav** (`.is-rail`, focus mode): 64px column with 38x38 icon buttons. Used in the reader's focus mode where the full nav is too much chrome.

### Reader (signature)

The chapter-reader view is the design's centerpiece and earns dedicated language.

- **Reader header**: chapter number (uppercase tracked label) → title (Source Serif 4 / 800 / 32px) → optional lede (italic serif, muted). Generous spacing (32–40px) before the body begins.
- **Reader body** (`.is-reader-body`): Source Serif 4, 17px, 1.75 line-height, color `text-main`, max-width 720px. The first paragraph has a serif drop cap in `Ledger Terracotta` (3.4em, line-height 0.85, floated). Blockquotes are italic muted text with a 2px primary border-left (the one acceptable border-stripe use, see Do's; it is editorial convention, not a card stripe).
- **Reader meta strip** (`.is-reader-meta`): uppercase 11px labels (chapter time, rating) sit above the title in `text-faint`. Quiet.
- **Reader navigation** (`.reader-nav`, `chapter-nav`): previous/next chapter links sit at the bottom, separated by a thin border-light rule. Always two-button row; never a single floating button.

### Book Cover (signature)

Book covers are the protagonist of the catalog. They are 4:5 aspect, 9–12px radius, with a warm `accent-light` placeholder background. The placeholder art is a vertical Nastaliq title in `Ledger Terracotta` (top-left or right-aligned), with a small uppercase Latin author label below. A `Cover` component renders this consistently across discover, shelf, row cards, and the continue rail; sizes scale from 52px (continue thumb) to 200px (book detail hero).

## 6. Do's and Don'ts

### Do:

- **Do tint every neutral toward the brand hue.** Cream warms toward terracotta; dark warms toward worn mahogany. Cool grays are prohibited.
- **Do use Source Serif 4 only for content the user is actually reading.** Chapter title, body, lede, blockquote, drop cap, stat numerals. Never in chrome.
- **Do let Nastaliq set the rhythm in any view that contains it.** Line-height 1.4 floor; vertical breathing room over horizontal density. `dir="rtl" lang="ur"` per element.
- **Do route active states through the background tint and color shift, not stripes.** `Ledger Terracotta Soft` background + `Ledger Terracotta` text + weight 600 is the canonical active state.
- **Do tint shadows warm** (`rgba(40, 30, 20, ...)`) on the parchment theme. Black shadows on cream read as SaaS-dashboard cards.
- **Do treat the four themes as content, not chrome.** Theme choice is an input to the design; every theme is validated for contrast independently.
- **Do honor `prefers-reduced-motion`** on every state transition (nav, theme, hover, rail). Motion is never the only way state changes.
- **Do use the gilt (warm yellow) only for ratings and highlights.** It is semantic, not decorative.
- **Do keep buttons quiet.** 7×12 padding, 13px body, no uppercase, no letter-spacing. The reading room does not contain stadium signage.
- **Do use the Cover component** for any book artwork; never inline cover-styling. The 4:5 ratio, warm placeholder, and Nastaliq title are part of the brand.

### Don't:

- **Don't use `#fff` on the parchment theme.** Fresh Folio (`#fffdf9`) is the brightest surface. Pure white reads as a foreign body on warm cream.
- **Don't use cool grays anywhere.** A neutral that reads as "GitHub gray" or "tech gray" is wrong by definition. Re-tone warmer.
- **Don't add a second brand hue.** The primary is a warm red-orange in all four themes. Blue accents, green confirmations, red danger swatches: prohibited. The system has one voice.
- **Don't add side-stripe borders.** `border-left` or `border-right` greater than 1px as a colored accent on cards, list items, or callouts is prohibited (per impeccable's shared design law). The existing 3px stripe on the active nav item violates this and is being phased out; new components should use background-tint + color shift only. **Exception:** the 2px blockquote `border-left` inside `.is-reader-body` is editorial convention and stays; this is the only acceptable in-content stripe.
- **Don't use gradient text.** No `background-clip: text` decorative gradients. Emphasis comes from weight, size, and color.
- **Don't reach for glassmorphism.** Backdrop-blur is not in the vocabulary. The Islands metaphor is solid surfaces on a tinted page, not glass.
- **Don't build a hero-metric template.** Big number + small label + supporting stats + gradient accent is the SaaS cliché. Profile stats are deliberately small (22px) and use the serif numeral; they are evidence, not display.
- **Don't make Musanif look like Rekhta dot org.** No web-1.0 catalog density, no above-the-fold-everything hero strips, no gray-on-gray secondary chrome.
- **Don't make Musanif look like Kindle / Apple Books / Google Play Books.** No library-as-database visuals, no white cards on gray, no generic e-reader chrome.
- **Don't make Musanif look like Goodreads or StoryGraph.** No follower counts, no review-noise feeds, no avatar-first chrome. This is a reading room, not a social network.
- **Don't add modals as a first thought.** Exhaust inline and progressive alternatives. The Islands metaphor means most "modal" content can live in a sibling island or a right-rail panel.
- **Don't animate layout properties.** Use `transform` and `opacity` only. State changes are 100–150ms ease-out; no bounce, no elastic, no choreography.
- **Don't write copy with em dashes.** Commas, colons, semicolons, periods, or parentheses. Also not `--`.
