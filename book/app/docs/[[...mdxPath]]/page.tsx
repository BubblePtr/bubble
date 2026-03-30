import { generateStaticParamsFor, importPage } from "nextra/pages";
import { useMDXComponents as getMDXComponents } from "../../../mdx-components";

const getStaticParams = generateStaticParamsFor("mdxPath");

function withDocsPrefix(mdxPath?: string[]) {
  return ["docs", ...(mdxPath ?? [])];
}

export async function generateStaticParams() {
  const params = await getStaticParams();

  return params
    .map((entry) => entry.mdxPath)
    .filter((path): path is string[] => Array.isArray(path) && path[0] === "docs")
    .map((path) => ({
      mdxPath: path.slice(1)
    }));
}

export async function generateMetadata(props: {
  params: Promise<{ mdxPath?: string[] }>;
}) {
  const params = await props.params;
  const { metadata } = await importPage(withDocsPrefix(params.mdxPath));

  return metadata;
}

const Wrapper = getMDXComponents({}).wrapper;

export default async function Page(props: {
  params: Promise<{ mdxPath?: string[] }>;
}) {
  const params = await props.params;
  const { default: MDXContent, toc, metadata, sourceCode } = await importPage(
    withDocsPrefix(params.mdxPath)
  );

  return (
    <Wrapper toc={toc} metadata={metadata} sourceCode={sourceCode}>
      <MDXContent {...props} params={params} />
    </Wrapper>
  );
}
