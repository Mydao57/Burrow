import type { ReactNode } from "react";

import { TextArea, TextField } from "@/shared/ui";

import type { EntryFormValues } from "../model/form";

interface Props {
  values: EntryFormValues;
  onChange: (patch: Partial<EntryFormValues>) => void;
  /** Shown instead of a plain placeholder when editing (password is not prefilled). */
  passwordHint?: string;
  /** Slot for features that help fill the password (e.g. a generator). */
  passwordTools?: ReactNode;
}

export function EntryFields({ values, onChange, passwordHint, passwordTools }: Props) {
  return (
    <>
      <TextField label="Title" value={values.title} onChange={(e) => onChange({ title: e.target.value })} required autoFocus />
      <TextField label="Username" value={values.username} onChange={(e) => onChange({ username: e.target.value })} />
      <TextField
        label="Password"
        type="text"
        value={values.password}
        hint={passwordHint}
        onChange={(e) => onChange({ password: e.target.value })}
        autoCapitalize="off"
      />
      {passwordTools}
      <TextField label="URL" value={values.url} onChange={(e) => onChange({ url: e.target.value })} />
      <TextArea label="Notes" value={values.notes} onChange={(e) => onChange({ notes: e.target.value })} />
    </>
  );
}
