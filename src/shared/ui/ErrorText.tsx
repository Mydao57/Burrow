import "./ui.css";

export function ErrorText({ message }: { message: string | null }) {
  return message ? (
    <p className="ui-error" role="alert">
      {message}
    </p>
  ) : null;
}
