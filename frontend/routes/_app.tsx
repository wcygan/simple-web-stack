import { type ComponentChildren, type ComponentType } from "preact";

interface MyAppProps {
  Component: ComponentType<unknown>; // A generic Preact component
  // Add other props if _app needs them from ctx.state, etc.
}

export default function App({ Component }: MyAppProps) {
  return (
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Todo App</title>
        <link rel="stylesheet" href="/styles.css" />
      </head>
      <body class="fresh-gradient min-h-screen">
        <Component />
      </body>
    </html>
  );
}
