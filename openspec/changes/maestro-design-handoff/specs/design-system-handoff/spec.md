## ADDED Requirements

### Requirement: Design tokens SHALL match the exported prototype

The application SHALL define CSS custom properties extracted from `design/maestro-desktop-prototype.html` for background, surface, foreground, muted text, border, accent, success, warning, danger, radius, shadow, spacing, type scale, and motion. Tokens SHALL use OKLCH values from the prototype unless a documented sRGB fallback is required for rendering.

#### Scenario: Dark theme tokens loaded

- **WHEN** `ApplicationSettings.theme` is `dark`
- **THEN** the document root SHALL expose prototype-equivalent OKLCH token values for surfaces and accent color

#### Scenario: Light theme tokens loaded

- **WHEN** `ApplicationSettings.theme` is `light`
- **THEN** the document root SHALL apply the prototype `.light-mode` token set

### Requirement: Shared UI primitives SHALL follow prototype styling

Buttons, text inputs, search fields, cards, section titles, tabs, modals, and checkboxes SHALL use shared CSS classes derived from the prototype (height, padding, border-radius, hover/focus/active/disabled states).

#### Scenario: Primary button appearance

- **WHEN** a primary action button is rendered
- **THEN** it SHALL use accent background, white foreground, and prototype border-radius without framework-default styling

#### Scenario: Focus visible on interactive controls

- **WHEN** the user focuses a button or input via keyboard
- **THEN** a visible focus treatment matching prototype `--border-focus` SHALL appear

### Requirement: Responsive layout SHALL honor the design manifest viewports

Layouts SHALL adapt across the viewport matrix in `design/DESIGN-MANIFEST.json` (360×800 through 1920×1080) without horizontal overflow on any listed size.

#### Scenario: Laptop viewport hub

- **WHEN** the viewport is 1366×768
- **THEN** the session hub toolbar and card list SHALL remain usable without horizontal scroll

#### Scenario: Mobile compact drawer

- **WHEN** the viewport is 360×800
- **THEN** navigation SHALL use the drawer pattern and main content SHALL not overflow horizontally
