import type { Metadata } from "next";

import "./globals.css";

export const metadata: Metadata = {
  title: "Sentinel Forge",
  description:
    "Security and verification infrastructure for Soroban smart contracts.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
