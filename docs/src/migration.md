# Migrating Eww Configuration

Most of the configuration can be ported over directly. But the following should be kept in mind for it to work:

#### Regarding yuck:

- The `eww.(css/scss/yuck)` files should be renamed to `ewwii.(css/scss/yuck)`.
- **MAGIC VARIABLES LIKE 'EWW_CPU' ARE NOT PRESENT**. So you would have to implement them manually using external scripts.
- Some widgets (like `center-box`) that are present in eww might be missing in ewwii, so it is recommended to refer to the widget name and properties defined in [ewwii_docs/widgets/props](https://ewwii-sh.github.io/docs/widgets/props).

#### Regarding styling:

- The `eventbox` widget in ewwii cannot be styled using `eventbox { styles }` in css/scss. To style them, adding a class is mandatory.
- Some styling might not work as ewwii uses GTK4 instead of GTK3, which eww uses. But most of the time, it will port over nicely.