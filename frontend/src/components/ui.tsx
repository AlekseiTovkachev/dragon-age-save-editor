import type {
  ChangeEvent,
  InputHTMLAttributes,
  LabelHTMLAttributes,
  ReactNode,
  SelectHTMLAttributes,
} from "react";

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

type NumericInputProps = Omit<TextInputProps, "inputMode" | "onChange"> & {
  allowDecimal?: boolean;
  max?: number;
  min?: number;
  onChange: (event: ChangeEvent<HTMLInputElement>) => void;
};

type NumericInputConstraints = Pick<NumericInputProps, "allowDecimal" | "max" | "min">;

function isAllowedNumericValue(value: string, { allowDecimal = false, min, max }: NumericInputConstraints) {
  if (value === "") {
    return true;
  }
  const pattern = allowDecimal ? /^\d+(?:\.\d*)?$/ : /^\d+$/;
  if (!pattern.test(value)) {
    return false;
  }
  const parsed = Number(value);
  if (!Number.isFinite(parsed)) {
    return false;
  }
  if (min !== undefined && parsed < min) {
    return false;
  }
  if (max !== undefined && parsed > max) {
    return false;
  }
  return true;
}

export function NumericInput({ allowDecimal = false, min, max, onChange, ...props }: NumericInputProps) {
  return (
    <TextInput
      {...props}
      inputMode={allowDecimal ? "decimal" : "numeric"}
      onChange={(event) => {
        if (isAllowedNumericValue(event.target.value, { allowDecimal, min, max })) {
          onChange(event);
        }
      }}
    />
  );
}

type SelectInputProps = SelectHTMLAttributes<HTMLSelectElement>;

export function SelectInput({ children, ...props }: SelectInputProps) {
  return <select {...props}>{children}</select>;
}

export function ButtonRow({ children, className = "" }: { children: ReactNode; className?: string }) {
  return <div className={["button-row", className].filter(Boolean).join(" ")}>{children}</div>;
}

type GridTableProps = {
  header: ReactNode;
  body: ReactNode;
  footer?: ReactNode;
  className?: string;
  bodyClassName?: string;
};

export function GridTable({ header, body, footer, className = "", bodyClassName = "" }: GridTableProps) {
  return (
    <div className={["grid-table", className].filter(Boolean).join(" ")}>
      {header}
      <ScrollRegion className={["grid-table-body", bodyClassName].filter(Boolean).join(" ")}>
        {body}
      </ScrollRegion>
      {footer}
    </div>
  );
}

type GridTableRowProps = {
  children: ReactNode;
  className?: string;
};

export function GridTableRow({ children, className = "" }: GridTableRowProps) {
  return <div className={["grid-table-row", className].filter(Boolean).join(" ")}>{children}</div>;
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
