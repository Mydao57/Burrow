import { useState } from "react";

import { errorMessage, generatePassword, type GeneratorOptions } from "@/shared/api";
import { Button, ErrorText } from "@/shared/ui";

import "./generator.css";

interface Props {
  /** Receives the generated password so the caller can put it in its form. */
  onGenerate: (password: string) => void;
}

const DEFAULTS: GeneratorOptions = {
  length: 20,
  lowercase: true,
  uppercase: true,
  digits: true,
  symbols: true,
  exclude_ambiguous: false,
};

const SETS = [
  ["lowercase", "a–z"],
  ["uppercase", "A–Z"],
  ["digits", "0–9"],
  ["symbols", "!@#"],
  ["exclude_ambiguous", "No look-alikes (O/0, I/l/1)"],
] as const;

export function PasswordGenerator({ onGenerate }: Props) {
  const [opts, setOpts] = useState(DEFAULTS);
  const [error, setError] = useState<string | null>(null);

  async function generate() {
    try {
      onGenerate(await generatePassword(opts));
      setError(null);
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  return (
    <fieldset className="pwgen">
      <legend>Generator</legend>
      <label className="pwgen__length">
        Length: {opts.length}
        <input
          type="range"
          min={8}
          max={64}
          value={opts.length}
          onChange={(e) => setOpts({ ...opts, length: Number(e.target.value) })}
        />
      </label>
      <div className="pwgen__sets">
        {SETS.map(([key, label]) => (
          <label key={key}>
            <input type="checkbox" checked={opts[key]} onChange={(e) => setOpts({ ...opts, [key]: e.target.checked })} />
            {label}
          </label>
        ))}
      </div>
      <ErrorText message={error} />
      <Button onClick={generate}>Generate</Button>
    </fieldset>
  );
}
