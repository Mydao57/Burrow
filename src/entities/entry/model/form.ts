import type { EntryDetail, EntryInput } from "@/shared/api";

export interface EntryFormValues {
  title: string;
  username: string;
  password: string;
  url: string;
  notes: string;
}

export const emptyEntryForm: EntryFormValues = {
  title: "",
  username: "",
  password: "",
  url: "",
  notes: "",
};

export function formFromEntry(entry: EntryDetail): EntryFormValues {
  // The password is intentionally not prefilled; empty means "keep current".
  return { ...emptyEntryForm, title: entry.title, username: entry.username, url: entry.url, notes: entry.notes };
}

export function toInput(values: EntryFormValues, id?: string): EntryInput {
  return { id, ...values, password: values.password || undefined };
}
