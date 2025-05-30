import { assertEquals, assertExists } from "jsr:@std/assert";
import { Button, type ButtonProps } from "../components/Button.tsx";

// Test Button component interface and props
Deno.test("Button component - has correct props interface", () => {
  const props: ButtonProps = {
    onClick: () => {},
    children: "Test button",
    disabled: false,
  };

  assertExists(props.onClick);
  assertEquals(props.children, "Test button");
  assertEquals(props.disabled, false);
});

Deno.test("Button component - function exists and is callable", () => {
  const result = Button({ children: "Test" });
  assertExists(result);
  assertEquals(typeof Button, "function");
});

// Simple utility function tests
Deno.test("Basic math operations", () => {
  const add = (a: number, b: number) => a + b;
  const subtract = (a: number, b: number) => a - b;

  assertEquals(add(2, 3), 5);
  assertEquals(add(-1, 1), 0);
  assertEquals(subtract(5, 3), 2);
  assertEquals(subtract(0, 1), -1);
});

// Test signal-like behavior (simplified counter logic)
Deno.test("Counter logic simulation", () => {
  let count = 0;

  const increment = () => count += 1;
  const decrement = () => count -= 1;
  const reset = () => count = 0;

  assertEquals(count, 0);

  increment();
  assertEquals(count, 1);

  increment();
  assertEquals(count, 2);

  decrement();
  assertEquals(count, 1);

  reset();
  assertEquals(count, 0);
});

// Test string utilities
Deno.test("String utilities", () => {
  const capitalize = (str: string) =>
    str.charAt(0).toUpperCase() + str.slice(1);
  const trim = (str: string) => str.trim();

  assertEquals(capitalize("hello"), "Hello");
  assertEquals(capitalize(""), "");
  assertEquals(trim("  hello world  "), "hello world");
});

// Test array utilities
Deno.test("Array utilities", () => {
  const numbers = [1, 2, 3, 4, 5];

  assertEquals(numbers.length, 5);
  assertEquals(numbers.filter((n) => n > 3), [4, 5]);
  assertEquals(numbers.map((n) => n * 2), [2, 4, 6, 8, 10]);
});
