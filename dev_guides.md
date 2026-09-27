# Developer Guides & Component Architecture

This document outlines the architecture, constraints, and coding standards for developing and extending **my-slint-kit**.

---

## 🏛️ Component Creation Guidelines

When building a new component for the toolkit, follow these rules:

1. **Do Not Inherit from `Window`**: 
   All components must inherit from `Rectangle` (or a structural layout/element like `Flickable`, `TouchArea`, etc.). Only top-level showcase wrappers or application shells should inherit from `Window`. This ensures widgets remain modular and can be embedded.
   
2. **Utilize Global Tokens**:
   Never hardcode dimensions, color hex values, or animation durations. Always bind properties to the `Theme` global:
   ```slint
   import { Theme } from "../tokens.slint";
   
   export component MyCustomWidget inherits Rectangle {
       background: Theme.palette.surface;
       border-radius: 4px;
       border-color: Theme.palette.border;
       border-width: 1px;
       // ...
   }
   ```

3. **Expose In/Out Properties**:
   Expose states and model values using `in-out` or `in` properties so that host applications can bind to them programmatically.

---

## ⚠️ State Management & Compiler Quirks

### The Non-Primitive Cast Constraint (`() as f32` error)
Slint has strict restrictions regarding expressions inside `states` blocks. 
If you assign calculated values (e.g., calculations involving coordinates, layouts, or nested elements) inside a state transition, the compiler may emit a cryptic error: `non-primitive cast: () as f32`.

**Avoid doing this:**
```slint
// May cause compiler error in state transitions if evaluated dynamically
states [
    active when root.checked : {
        indicator.x: root.width - indicator.width - Theme.spacing.xs; 
    }
]
```

**Preferred approach (Reactive Properties & Conditionals):**
Use inline conditional expressions or bind properties directly to reactive calculations. The component will automatically animate if `animate` is specified on the property:
```slint
export component Switch inherits Rectangle {
    in-out property <bool> checked: false;
    
    // Animate x position reactively
    indicator.x: checked ? (root.width - indicator.width - Theme.spacing.xs) : Theme.spacing.xs;
    
    animate indicator.x {
        duration: Theme.animation.fast;
        easing: ease-in-out;
    }
}
```

---

## 🔄 Avoiding Import Cycles

To prevent circular dependencies within Slint compilation:

1. **Isolate Showcase Imports**:
   - Component files (`ui/components/*.slint`) must **never** import other components unless there is a strict parent-child relationship (e.g., `Accordion` importing `AccordionItem`).
   - Component files must only import `tokens.slint` and `icons.slint`.
2. **Individual Imports in Examples**:
   - Files in `ui/examples/` must import components directly from their source files rather than an umbrella export (e.g., import `{ Button } from "../components/button.slint"` instead of importing from `preview.slint` or a unified export file).
   - This keeps dependency trees flat and compile times fast.

---

## 🧪 Testing & Verification

1. **Compile Checks**:
   Always run `cargo check` before committing any Slint changes to catch syntax and layout property warnings early.
2. **Interactive Showcase Validation**:
   - When introducing or modifying a component, ensure its corresponding example file in `ui/examples/` is updated.
   - Run the showcase (`cargo run --bin preview`) and test the component in both **Light Mode** and **Dark Mode** to check text contrast, hover states, and focus styling.
