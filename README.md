# Yucky Ewwii!

This ewwii plugin provides native yuck support. It is useful for users migrating from eww or for users who prefer lisp-like languages in general.

## Migrating Eww Configuration

Most of the configuration can be ported over directly. But the following should be kept in mind for it to work:

#### Regarding yuck:

- The `eww.(css/scss/yuck)` files should be renamed to `ewwii.(css/scss/yuck)`.
- **MAGIC VARIABLES LIKE 'EWW_CPU' ARE NOT PRESENT**. So you would have to implement them manually using external scripts.
- Some widgets (like `center-box`) that are present in eww might be missing in ewwii, so it is recommended to refer to the widget name and properties defined in [ewwii_docs/widgets/props](https://ewwii-sh.github.io/docs/widgets/props).

#### Regarding styling:

- The `eventbox` widget in ewwii cannot be styled using `eventbox { styles }` in css/scss. To style them, adding a class is mandatory.
- Some styling might not work as ewwii uses GTK4 instead of GTK3, which eww uses. But most of the time, it will port over nicely.

## Versioning Notice

This project uses a custom versioning system instead of the standard [Semantic Versioning](https://semver.org/) to avoid confusion.

It uses a **EWWII_VERSION.RELEASE_NUMBER** system where:

- **EWWII_VERSION**: The version of ewwii this release was made for.
- **RELEASE_NUMBER**: The patch number of this plugin.