import type { InputHTMLAttributes, ReactNode, TextareaHTMLAttributes } from "react";
import { useId } from "react";

import "./ui.css";

interface FieldProps {
  label: string;
  hint?: string;
  action?: ReactNode;
}

export function TextField({
  label,
  hint,
  action,
  ...input
}: FieldProps & InputHTMLAttributes<HTMLInputElement>) {
  const id = useId();
  return (
    <div className="ui-field">
      <label htmlFor={id}>{label}</label>
      <div className="ui-field__row">
        <input id={id} autoComplete="off" spellCheck={false} {...input} />
        {action}
      </div>
      {hint && <small>{hint}</small>}
    </div>
  );
}

export function TextArea({ label, hint, ...area }: FieldProps & TextareaHTMLAttributes<HTMLTextAreaElement>) {
  const id = useId();
  return (
    <div className="ui-field">
      <label htmlFor={id}>{label}</label>
      <textarea id={id} rows={4} {...area} />
      {hint && <small>{hint}</small>}
    </div>
  );
}
