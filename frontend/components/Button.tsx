import { JSX } from "preact";

// Explicitly include type and disabled attributes for buttons
interface ButtonProps extends JSX.HTMLAttributes<HTMLButtonElement> {
  type?: "button" | "submit" | "reset";
  disabled?: boolean; // Explicitly adding disabled here
}

export function Button(props: ButtonProps) {
  return (
    <button
      {...props}
      type={props.type || "button"} // Default to "button" if not specified
      // The disabled attribute will be passed through by {...props}
      class={`px-3 py-2 bg-blue-500 text-white rounded hover:bg-blue-700 disabled:opacity-50 ${props.class ?? ""}`}
    />
  );
}
