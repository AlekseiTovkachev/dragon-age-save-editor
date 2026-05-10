import type { ReactNode } from "react";
import { TitleBar } from "./TitleBar";

type AppShellProps = {
  sidebar: ReactNode;
  children: ReactNode;
};

export function AppShell({ sidebar, children }: AppShellProps) {
  return (
    <div className="app-frame">
      <TitleBar />
      <div className="app">
        {sidebar}
        <main className="app-main">{children}</main>
      </div>
    </div>
  );
}
