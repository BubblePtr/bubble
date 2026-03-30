import type { Metadata } from "next";
import { Head } from "nextra/components";
import { getPageMap } from "nextra/page-map";
import { Footer, Layout, Navbar } from "nextra-theme-docs";
import "nextra-theme-docs/style.css";

export const metadata: Metadata = {
  title: "Learn Agent Build",
  description: "Learn Agent Build 的架构与实现思路教学文档。"
};

const navbar = <Navbar logo={<b>Learn Agent Build</b>} />;
const footer = <Footer>MIT {new Date().getFullYear()} © Learn Agent Build.</Footer>;

export default async function RootLayout({
  children
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="zh-CN" dir="ltr" suppressHydrationWarning>
      <Head />
      <body>
        <Layout
          navbar={navbar}
          footer={footer}
          pageMap={await getPageMap()}
          docsRepositoryBase="https://github.com/BubblePtr/bubble/tree/main/book/content"
        >
          {children}
        </Layout>
      </body>
    </html>
  );
}
