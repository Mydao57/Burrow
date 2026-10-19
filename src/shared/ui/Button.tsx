import type { ButtonHTMLAttributes } from "react";

import { Icon, type IconName } from "./Icon";

import "./ui.css";

type Variant = "primary" | "secondary" | "danger" | "ghost";

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant;
  size?: "md" | "sm";
  icon?: IconName;
  /** Icon-only button: `aria-label` (or `title`) is required for accessibility. */
  iconOnly?: boolean;
}

export function Button({
  variant = "secondary",
  size = "md",
  icon,
  iconOnly = false,
  className = "",
  type = "button",
  children,
  ...rest
}: Props) {
  const classes = [
    "ui-btn",
    `ui-btn--${variant}`,
    size === "sm" && "ui-btn--sm",
    iconOnly && "ui-btn--icon",
    className,
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <button type={type} className={classes} {...rest}>
      {icon && <Icon name={icon} size={size === "sm" ? 14 : 16} />}
      {children}
    </button>
  );
}
