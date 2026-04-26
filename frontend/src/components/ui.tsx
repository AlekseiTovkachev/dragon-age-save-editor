import type { ButtonHTMLAttributes, InputHTMLAttributes, LabelHTMLAttributes, ReactNode, SelectHTMLAttributes } from "react";

type PanelProps = {
  title?: ReactNode;
  headingAction?: ReactNode;
  children: ReactNode;
  className?: string;
  scroll?: boolean;
};

export function Panel({ title, headingAction, children, className = "", scroll = false }: PanelProps) {
  const classes = ["panel", className, scroll ? "scroll-panel" : ""].filter(Boolean).join(" ");
  return (
    <section className={classes}>
      {title || headingAction ? (
        <div className="panel-heading">
          {typeof title === "string" ? <h2>{title}</h2> : title}
          {headingAction}
        </div>
      ) : null}
      {children}
    </section>
  );
}

export function PanelBody({ children, className = "" }: { children: ReactNode; className?: string }) {
  return <div className={["panel-scroll-body", className].filter(Boolean).join(" ")}>{children}</div>;
}

export function ScrollRegion({ children, className = "" }: { children: ReactNode; className?: string }) {
  return <div className={["scroll-region", className].filter(Boolean).join(" ")}>{children}</div>;
}

export function EmptyState({ children }: { children: ReactNode }) {
  return <p className="muted">{children}</p>;
}

type ListRowProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  active?: boolean;
};

export function ListRow({ active = false, className = "", children, ...props }: ListRowProps) {
  const classes = ["list-row", active ? "active" : "", className].filter(Boolean).join(" ");
  return (
    <button className={classes} {...props}>
      {children}
    </button>
  );
}

type FieldProps = LabelHTMLAttributes<HTMLLabelElement> & {
  label: ReactNode;
};

export function Field({ label, children, className = "", ...props }: FieldProps) {
  return (
    <label className={className || undefined} {...props}>
      <span>{label}</span>
      {children}
    </label>
  );
}

export function FieldGrid({ children, className = "" }: { children: ReactNode; className?: string }) {
  return <div className={["field-grid", className].filter(Boolean).join(" ")}>{children}</div>;
}

type SectionCardProps = {
  title?: ReactNode;
  children: ReactNode;
  className?: string;
  headingLevel?: 3 | 4;
};

export function SectionCard({ title, children, className = "", headingLevel = 3 }: SectionCardProps) {
  const Heading = headingLevel === 4 ? "h4" : "h3";
  return (
    <section className={["section-card", className].filter(Boolean).join(" ")}>
      {title ? <Heading>{title}</Heading> : null}
      {children}
    </section>
  );
}

type TextInputProps = InputHTMLAttributes<HTMLInputElement>;

export function TextInput(props: TextInputProps) {
  return <input {...props} />;
}

type SelectInputProps = SelectHTMLAttributes<HTMLSelectElement>;

export function SelectInput({ children, ...props }: SelectInputProps) {
  return <select {...props}>{children}</select>;
}

export function ButtonRow({ children, className = "" }: { children: ReactNode; className?: string }) {
  return <div className={["button-row", className].filter(Boolean).join(" ")}>{children}</div>;
}

type ChoiceRowProps = LabelHTMLAttributes<HTMLLabelElement> & {
  kind: "checkbox" | "radio";
};

export function ChoiceRow({ kind, children, className = "", ...props }: ChoiceRowProps) {
  const rowClass = kind === "checkbox" ? "check-row" : "radio-row";
  return (
    <label className={[rowClass, className].filter(Boolean).join(" ")} {...props}>
      {children}
    </label>
  );
}
