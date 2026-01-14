# Dioxus Blocks UI

## Serving Your App

Tailwind CSS

```bash
pnpx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of your project to start developing with the default platform:

```bash
cd dioxus-blocks-ui

dx serve -p dioxus-blocks-ui --platform web

dx serve -p dioxus-blocks-ui --platform web --port 8080
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
cd dioxus-blocks-ui

dx serve -p dioxus-blocks-ui --platform desktop
```
