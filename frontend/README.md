# Fresh project

Your new Fresh project is ready to go. You can follow the Fresh "Getting
Started" guide here: https://fresh.deno.dev/docs/getting-started

### Usage

Make sure to install Deno:
https://docs.deno.com/runtime/getting_started/installation

Then start the project in development mode:

```
deno task dev
```

This will watch the project directory and restart as necessary.

### Testing

Run the test suite:

```
deno task test
```

This runs all tests in the `tests/` directory using Deno's built-in test runner.
The tests include:

- Component interface validation
- Basic utility function tests
- Counter logic simulation
- String and array utilities

To run tests with additional options:

```
deno test --watch  # Watch mode for test-driven development
deno test --coverage  # Run with coverage reporting
```

### Available Tasks

- `deno task dev` - Start development server with hot reloading
- `deno task build` - Build the project for production
- `deno task start` - Start the production server
- `deno task check` - Format, lint, and type-check the code
- `deno task test` - Run the test suite
- `deno task update` - Update Fresh framework
