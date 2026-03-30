import { Head } from "nextra/components";
import { getPageMap } from "nextra/page-map";
import { Footer, Layout, Navbar } from "nextra-theme-docs";
import "nextra-theme-docs/style.css";

const navbar = <Navbar logo={<b>Learn Agent Build</b>} />;
const footer = <Footer>MIT {new Date().getFullYear()} © Learn Agent Build.</Footer>;

export default async function DocsLayout({
  children
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>
      <Head />
      <Layout
        navbar={navbar}
        footer={footer}
        pageMap={await getPageMap("/docs")}
        docsRepositoryBase="https://github.com/BubblePtr/bubble/tree/main/book/content/docs"
      >
        {children}
      </Layout>
    </>
  );
}
