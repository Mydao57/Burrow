import type { ButtonHTMLAttributes } from "react";

import "./ui.css";

type Variant = "primary" | "secondary" | "danger" | "ghost";

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant;
}

export function Button({ variant = "secondary", className = "", type = "button", ...rest }: Props) {
  return <button type={type} className={`ui-btn ui-btn--${variant} ${className}`.trim()} {...rest} />;
}
