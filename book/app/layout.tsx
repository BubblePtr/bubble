import type { Metadata, Viewport } from "next";
import { Fustat, Inter } from "next/font/google";
import "./globals.css";
import "remixicon/fonts/remixicon.css";

export const metadata: Metadata = {
  title: "Learn Agent Build",
  description: "Learn Agent Build 的架构与实现思路教学文档。"
};

export const viewport: Viewport = {
  themeColor: [
    { media: "(prefers-color-scheme: light)", color: "rgb(250,250,250)" },
    { media: "(prefers-color-scheme: dark)", color: "rgb(17,17,17)" }
  ]
};

const fustat = Fustat({
  subsets: ["latin"],
  variable: "--font-display",
  weight: ["700", "800"]
});

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-body",
  weight: ["400", "500", "600", "700"]
});

export default async function RootLayout({
  children
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="zh-CN" dir="ltr" suppressHydrationWarning>
      <body className={`${fustat.variable} ${inter.variable}`}>
        {children}
      </body>
    </html>
  );
}
