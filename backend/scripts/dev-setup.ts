#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-env

import { cyan, red, green, bold, magenta } from "@std/fmt/colors";

async function runCommand(label: string, cmd: string, args: string[] = []): Promise<void> {
    console.log(cyan(`🔧 ${label}...`));
    const command = new Deno.Command(cmd, {
        args,
        stdout: "inherit",
        stderr: "inherit",
    });
    const status = await command.output();
    if (!status.success) {
        console.error(red(`❌ ${label} failed.`));
        Deno.exit(1);
    }
    console.log(green(`✅ ${label} complete.`));
}

async function main() {
    console.log(bold(magenta("🚀 Setting up backend development environment...")));

    await runCommand("Installing cargo-audit", "cargo", ["install", "cargo-audit"]);
    await runCommand("Installing cargo-deny", "cargo", ["install", "cargo-deny"]);
    await runCommand("Installing cargo-watch", "cargo", ["install", "cargo-watch"]);
    // Pre-commit hooks are skipped as per request

    console.log(bold(green("🎉 Backend development environment ready!")));
}

if (import.meta.main) {
    main().catch((err) => {
        console.error(red("Error during setup:"), err);
        Deno.exit(1);
    });
} 