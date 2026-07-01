# Components — web-ui

C4 Level 3 components for `web-ui`.

## Base components

| Component | Pattern                                                    |
| --------- | ---------------------------------------------------------- |
| Button    | CVA variants, 6 + 2 OL variants, 8 + 1 OL sizes, `asChild` |
| Alert     | CVA variants, 3 + 3 OL semantic variants, `role="alert"`   |
| Input     | focus-visible, `aria-invalid`, 44 px OL touch target       |
| Card      | Subcomponents with `data-slot`                             |
| Label     | Radix Label                                                |
| Dialog    | Radix Dialog, portal, overlay, close button                |
| Textarea  | focus-visible, `aria-invalid`                              |
| Badge     | CVA variants                                               |

## OrganicLever components

| Component    | Props                                           | Description                               |
| ------------ | ----------------------------------------------- | ----------------------------------------- |
| Icon         | `name`, `size`, `filled`                        | 34-icon inline SVG set                    |
| Toggle       | `value`, `onChange`, `label`                    | Slide-switch, teal active state           |
| ProgressRing | `size`, `stroke`, `progress`, `color`, `bg`     | Circular SVG arc                          |
| Sheet        | `title`, `onClose`, `children`                  | Bottom-anchored modal, slide-up animation |
| AppHeader    | `title`, `subtitle`, `onBack`, `trailing`       | Back-button + title + optional trailing   |
| StatCard     | `label`, `value`, `unit`, `hue`, `icon`, `info` | Dashboard stat tile                       |
| InfoTip      | `title`, `text`                                 | Info button opening a Sheet               |
| HuePicker    | `value`, `onChange`                             | 6-hue swatch row                          |
| TabBar       | `tabs`, `current`, `onChange`                   | 60 px mobile bottom navigation            |
| SideNav      | `brand`, `tabs`, `current`, `onChange`          | 220 px desktop side navigation            |

See [../behavior/gherkin/](../behavior/gherkin/) for the behavioral specs backing each component.
See [component-web-ui.md](./component-web-ui.md) for the C4 component diagram placeholder.
