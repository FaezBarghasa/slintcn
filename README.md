# my-slint-kit (slintcn)

A premium UI component library for **Slint** and **Rust** that merges the modern aesthetics of **Shadcn UI** with the robust, embedded-friendly HMI components of **LVGL**.

This kit is designed to be highly responsive, pixel-perfect, and easily customizable, supporting dynamic light/dark theming, animation scaling, and runtime configuration options.

---

## 🚀 Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed. To run the interactive showcase and examples gallery:

```bash
# Run the live showcase of all components and example screens
cargo run --bin preview
```

To run with slowed-down animations (useful for visual debugging and micro-interaction checks):
```bash
SLINT_SLOW_ANIMATIONS=1 cargo run --bin preview
```

---

## 📂 Project Structure

The project is structured logically into components, layouts, token specifications, and example sheets:

```
slintcn/
├── Cargo.toml                  # Workspace and library manifest
├── build.rs                    # Slint compiler configuration
├── src/
│   ├── lib.rs                  # Library entrypoint including compiled Slint modules
│   └── theme.rs                # Theme control utility bindings (dark mode toggles)
├── ui/
│   ├── tokens.slint            # Design system tokens (colors, spacing, animations)
│   ├── preview.slint           # Primary interactive showcase container
│   ├── components/             # Reusable UI component definitions
│   │   ├── button.slint
│   │   ├── slider.slint
│   │   ├── arc.slint
│   │   └── ... (50 components)
│   └── examples/               # Example configurations & demo pages
│       ├── index.slint         # Gallery Hub aggregating all widget examples
│       ├── button_examples.slint
│       ├── slider_examples.slint
│       └── ...
```

---

## 🎨 Design System & Tokens

All components are bound to a central, reactive design system defined in [tokens.slint](file:///home/jrad/RustroverProjects/slintcn/ui/tokens.slint). 

- **Color Palette (`PaletteData`)**: Auto-adapts to `Theme.is_dark` at runtime. Includes semantic colors like `primary`, `surface`, `border`, `hover_overlay`, `success`, `warning`, and `critical`.
- **Spacing Scale (`SpacingData`)**: Consistent margins and padding (`xs` to `xl`).
- **Typography (`TypographyData`)**: Font definitions, sizes, and weights for headers, body, and caption text.
- **Animations (`AnimationData`)**: Transition durations scale programmatically (e.g., using `SLINT_SLOW_ANIMATIONS=1`).

---

## 🛠️ Ported Components Catalog

The kit contains **50 reusable UI components** split into core design categories:

### 🌟 Modern Web / Shadcn-style Components
- **Feedback & Overlays**: [Alert](file:///home/jrad/RustroverProjects/slintcn/ui/components/alert.slint), [AlertDialog](file:///home/jrad/RustroverProjects/slintcn/ui/components/alert_dialog.slint), [Dialog](file:///home/jrad/RustroverProjects/slintcn/ui/components/dialog.slint), [Drawer](file:///home/jrad/RustroverProjects/slintcn/ui/components/drawer.slint), [Sheet](file:///home/jrad/RustroverProjects/slintcn/ui/components/sheet.slint), [HoverCard](file:///home/jrad/RustroverProjects/slintcn/ui/components/hover_card.slint), [Tooltip](file:///home/jrad/RustroverProjects/slintcn/ui/components/tooltip.slint), [Toast](file:///home/jrad/RustroverProjects/slintcn/ui/components/toast.slint).
- **Navigation & Layouts**: [Breadcrumb](file:///home/jrad/RustroverProjects/slintcn/ui/components/breadcrumb.slint), [TabWidget](file:///home/jrad/RustroverProjects/slintcn/ui/components/tab_widget.slint), [ScrollArea](file:///home/jrad/RustroverProjects/slintcn/ui/components/scroll_area.slint).
- **Form Controls**: [Button](file:///home/jrad/RustroverProjects/slintcn/ui/components/button.slint), [Input](file:///home/jrad/RustroverProjects/slintcn/ui/components/input.slint), [Textarea](file:///home/jrad/RustroverProjects/slintcn/ui/components/textarea.slint), [Checkbox](file:///home/jrad/RustroverProjects/slintcn/ui/components/checkbox.slint), [Switch](file:///home/jrad/RustroverProjects/slintcn/ui/components/switch.slint), [Select](file:///home/jrad/RustroverProjects/slintcn/ui/components/select.slint), [Combobox](file:///home/jrad/RustroverProjects/slintcn/ui/components/combobox.slint).
- **Content Displays**: [Card](file:///home/jrad/RustroverProjects/slintcn/ui/components/card.slint), [Badge](file:///home/jrad/RustroverProjects/slintcn/ui/components/badge.slint), [Avatar](file:///home/jrad/RustroverProjects/slintcn/ui/components/avatar.slint), [Skeleton](file:///home/jrad/RustroverProjects/slintcn/ui/components/skeleton.slint), [AccordionItem](file:///home/jrad/RustroverProjects/slintcn/ui/components/accordion.slint), [Collapsible](file:///home/jrad/RustroverProjects/slintcn/ui/components/collapsible.slint), [Table](file:///home/jrad/RustroverProjects/slintcn/ui/components/table.slint).

### 🕹️ Embedded HMI / LVGL-style Components
- **Analog Values & Meters**: [Arc](file:///home/jrad/RustroverProjects/slintcn/ui/components/arc.slint), [Slider](file:///home/jrad/RustroverProjects/slintcn/ui/components/slider.slint), [ProgressBar](file:///home/jrad/RustroverProjects/slintcn/ui/components/progress_bar.slint), [Gauge](file:///home/jrad/RustroverProjects/slintcn/ui/components/gauge.slint), [RadialGauge](file:///home/jrad/RustroverProjects/slintcn/ui/components/radial_gauge.slint), [Led](file:///home/jrad/RustroverProjects/slintcn/ui/components/led.slint), [Scale](file:///home/jrad/RustroverProjects/slintcn/ui/components/scale.slint).
- **Embedded Input**: [ButtonMatrix](file:///home/jrad/RustroverProjects/slintcn/ui/components/button_matrix.slint), [Keypad](file:///home/jrad/RustroverProjects/slintcn/ui/components/keypad.slint), [PinPad](file:///home/jrad/RustroverProjects/slintcn/ui/components/pinpad.slint), [VirtualKeyboard](file:///home/jrad/RustroverProjects/slintcn/ui/components/keyboard.slint), [Roller](file:///home/jrad/RustroverProjects/slintcn/ui/components/roller.slint).
- **Visual indicators**: [Spinner](file:///home/jrad/RustroverProjects/slintcn/ui/components/spinner.slint), [Chart](file:///home/jrad/RustroverProjects/slintcn/ui/components/chart.slint), [RollingChart](file:///home/jrad/RustroverProjects/slintcn/ui/components/rolling_chart.slint), [SplitChart](file:///home/jrad/RustroverProjects/slintcn/ui/components/split_chart.slint).

---

## 🛠️ Development & Extending the Kit

If you want to add components or modify the theme configuration:
- Component definitions should reside in `ui/components/`.
- Ensure new components do not inherit from `Window` directly (inherit from `Rectangle` or basic containers) to allow embedding them modularly.
- Add components to the showcase window in [preview.slint](file:///home/jrad/RustroverProjects/slintcn/ui/preview.slint) and create corresponding interactive example panels in `ui/examples/` to test state transitions.
