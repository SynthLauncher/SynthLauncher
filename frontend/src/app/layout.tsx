import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "SynthLauncher",
  description: "SynthLauncher",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="flex flex-row">{children}</body>
    </html>
  );
}
