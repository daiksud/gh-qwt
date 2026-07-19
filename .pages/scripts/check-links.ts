const SITE_ORIGIN = "https://daiksud.github.io";
const BASE_PATH = "/gh-qwt/";
const BASE_ROOT = BASE_PATH.slice(0, -1);
const REPOSITORY_EDIT_ROOT = "/daiksud/gh-qwt/edit/";

const DIST_DIRECTORY = `${import.meta.dir}/../dist`;
const DOCS_DIRECTORY = `${import.meta.dir}/../../docs`;

type AttributeName = "href" | "src";

interface LinkReference {
  attribute: AttributeName;
  value: string;
}

interface Failure {
  source: string;
  reference?: LinkReference;
  message: string;
}

const failures: Failure[] = [];
const fileContents = new Map<string, Promise<string>>();
const anchorIds = new Map<string, Promise<Set<string>>>();

function addFailure(source: string, message: string, reference?: LinkReference): void {
  failures.push({ source, reference, message });
}

function decodeHtmlEntities(value: string): string {
  return value.replace(
    /&(?:#(\d+)|#x([\da-f]+)|(amp|apos|gt|lt|quot));/gi,
    (entity, decimal: string | undefined, hexadecimal: string | undefined, named: string | undefined) => {
      if (decimal !== undefined || hexadecimal !== undefined) {
        const codePoint = Number.parseInt(decimal ?? hexadecimal!, decimal === undefined ? 16 : 10);

        try {
          return String.fromCodePoint(codePoint);
        } catch {
          return entity;
        }
      }

      const namedEntities: Record<string, string> = {
        amp: "&",
        apos: "'",
        gt: ">",
        lt: "<",
        quot: '"',
      };

      return namedEntities[named!.toLowerCase()] ?? entity;
    },
  );
}

function withoutNonMarkupContent(html: string): string {
  return html
    .replace(/<!--[\s\S]*?-->/g, "")
    .replace(/(<script\b[^>]*>)[\s\S]*?<\/script\s*>/gi, "$1</script>")
    .replace(/(<style\b[^>]*>)[\s\S]*?<\/style\s*>/gi, "$1</style>");
}

function getAttributeValue(match: RegExpExecArray): string {
  return decodeHtmlEntities(match[2] ?? match[3] ?? match[4] ?? "");
}

function findLinkReferences(html: string): LinkReference[] {
  const markup = withoutNonMarkupContent(html);
  const attributePattern = /(?:^|[\s<])(href|src)\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s"'=<>`]+))/gi;
  const references: LinkReference[] = [];

  for (const match of markup.matchAll(attributePattern)) {
    references.push({
      attribute: match[1]!.toLowerCase() as AttributeName,
      value: getAttributeValue(match),
    });
  }

  return references;
}

function findAnchorIds(markup: string): Set<string> {
  const ids = new Set<string>();
  const idPattern = /(?:^|[\s<])id\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s"'=<>`]+))/gi;

  for (const match of markup.matchAll(idPattern)) {
    ids.add(decodeHtmlEntities(match[1] ?? match[2] ?? match[3] ?? ""));
  }

  const anchorPattern = /<a\b[^>]*>/gi;
  const namePattern = /(?:^|\s)name\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s"'=<>`]+))/i;
  for (const anchor of markup.matchAll(anchorPattern)) {
    const name = namePattern.exec(anchor[0]);
    if (name !== null) {
      ids.add(decodeHtmlEntities(name[1] ?? name[2] ?? name[3] ?? ""));
    }
  }

  return ids;
}

async function readOutputFile(relativePath: string): Promise<string> {
  const cachedContents = fileContents.get(relativePath);
  if (cachedContents !== undefined) {
    return cachedContents;
  }

  const contents = Bun.file(`${DIST_DIRECTORY}/${relativePath}`).text();
  fileContents.set(relativePath, contents);
  return contents;
}

async function getAnchorIds(relativePath: string): Promise<Set<string>> {
  let ids = anchorIds.get(relativePath);
  if (ids === undefined) {
    ids = readOutputFile(relativePath).then((contents) => findAnchorIds(withoutNonMarkupContent(contents)));
    anchorIds.set(relativePath, ids);
  }

  return ids;
}

async function scanFiles(directory: string, pattern: string): Promise<string[]> {
  const glob = new Bun.Glob(pattern);
  const paths: string[] = [];

  try {
    for await (const path of glob.scan({ cwd: directory, dot: true, onlyFiles: true })) {
      paths.push(path.replaceAll("\\", "/"));
    }
  } catch (error) {
    if (error instanceof Error && "code" in error && error.code === "ENOENT") {
      return [];
    }

    throw error;
  }

  return paths.sort();
}

function htmlPathToPublicPath(relativePath: string): string {
  if (relativePath === "index.html") {
    return BASE_PATH;
  }

  if (relativePath.endsWith("/index.html")) {
    return `${BASE_PATH}${relativePath.slice(0, -"index.html".length)}`;
  }

  return `${BASE_PATH}${relativePath}`;
}

function expectedEditTarget(relativeHtmlPath: string, sourceDocs: Set<string>): string | undefined {
  let target: string;
  if (relativeHtmlPath === "index.html") {
    target = "docs/README.md";
  } else if (relativeHtmlPath.endsWith("/index.html")) {
    target = `docs/${relativeHtmlPath.slice(0, -"index.html".length)}README.md`;
  } else {
    return undefined;
  }

  return sourceDocs.has(target) ? target : undefined;
}

function parseEditTarget(
  url: URL,
  source: string,
  reference: LinkReference,
  sourceDocs: Set<string>,
): string | undefined {
  if (url.hostname !== "github.com" || !url.pathname.startsWith(REPOSITORY_EDIT_ROOT)) {
    return undefined;
  }

  let pathname: string;
  try {
    pathname = decodeURIComponent(url.pathname);
  } catch {
    addFailure(source, "edit URL contains malformed percent encoding", reference);
    return "";
  }

  const match = /^\/daiksud\/gh-qwt\/edit\/main\/(docs\/(?:[^/]+\/)*README\.md)$/.exec(pathname);
  if (match === null || url.search !== "" || url.hash !== "") {
    addFailure(
      source,
      "edit URL must match https://github.com/daiksud/gh-qwt/edit/main/docs/**/README.md",
      reference,
    );
    return "";
  }

  const target = match[1]!;
  if (!sourceDocs.has(target)) {
    addFailure(source, `edit URL points to a missing source document: ${target}`, reference);
    return "";
  }

  return target;
}

function outputCandidates(pathname: string, source: string, reference: LinkReference): string[] | undefined {
  if (pathname !== BASE_ROOT && !pathname.startsWith(BASE_PATH)) {
    addFailure(source, `local URL escapes the ${BASE_PATH} base path`, reference);
    return undefined;
  }

  const encodedRelativePath = pathname === BASE_ROOT ? "" : pathname.slice(BASE_PATH.length);
  let relativePath: string;
  try {
    relativePath = decodeURIComponent(encodedRelativePath);
  } catch {
    addFailure(source, "local URL contains malformed percent encoding", reference);
    return undefined;
  }

  if (
    relativePath.includes("\\") ||
    relativePath.includes("\0") ||
    relativePath.split("/").some((segment) => segment === "." || segment === "..")
  ) {
    addFailure(source, "local URL resolves to an unsafe output path", reference);
    return undefined;
  }

  if (relativePath === "") {
    return ["index.html"];
  }

  if (relativePath.endsWith("/")) {
    return [`${relativePath}index.html`, `${relativePath.slice(0, -1)}.html`];
  }

  return [relativePath, `${relativePath}.html`, `${relativePath}/index.html`];
}

function fragmentIdentifier(hash: string): string | undefined {
  if (hash === "" || hash === "#") {
    return undefined;
  }

  const encodedFragment = hash.slice(1).split(":~:text=", 1)[0]!;
  if (encodedFragment === "") {
    return undefined;
  }

  return decodeURIComponent(encodedFragment);
}

async function validateLocalReference(
  url: URL,
  source: string,
  reference: LinkReference,
  outputFiles: Set<string>,
): Promise<void> {
  const candidates = outputCandidates(url.pathname, source, reference);
  if (candidates === undefined) {
    return;
  }

  const target = candidates.find((candidate) => outputFiles.has(candidate));
  if (target === undefined) {
    addFailure(source, `no generated target found (tried ${candidates.join(", ")})`, reference);
    return;
  }

  if (url.hash === "" || !(target.endsWith(".html") || target.endsWith(".svg"))) {
    return;
  }

  let identifier: string | undefined;
  try {
    identifier = fragmentIdentifier(url.hash);
  } catch {
    addFailure(source, "fragment contains malformed percent encoding", reference);
    return;
  }

  if (identifier === undefined) {
    return;
  }

  const ids = await getAnchorIds(target);
  if (!ids.has(identifier)) {
    addFailure(source, `fragment #${identifier} does not exist in ${target}`, reference);
  }
}

async function main(): Promise<void> {
  const [htmlPaths, allOutputPaths, sourceReadmePaths] = await Promise.all([
    scanFiles(DIST_DIRECTORY, "**/*.html"),
    scanFiles(DIST_DIRECTORY, "**/*"),
    scanFiles(DOCS_DIRECTORY, "**/README.md"),
  ]);

  if (htmlPaths.length === 0) {
    console.error(`No generated HTML found in ${DIST_DIRECTORY}. Run bun run build first.`);
    process.exitCode = 1;
    return;
  }

  const outputFiles = new Set(allOutputPaths);
  const sourceDocs = new Set(sourceReadmePaths.map((path) => `docs/${path}`));
  const seenEditTargets = new Set<string>();
  let localReferenceCount = 0;

  for (const relativeHtmlPath of htmlPaths) {
    const html = await readOutputFile(relativeHtmlPath);
    const pageUrl = new URL(htmlPathToPublicPath(relativeHtmlPath), SITE_ORIGIN);
    const pageEditTargets = new Set<string>();

    for (const reference of findLinkReferences(html)) {
      let url: URL;
      try {
        url = new URL(reference.value, pageUrl);
      } catch {
        addFailure(relativeHtmlPath, "URL cannot be parsed", reference);
        continue;
      }

      const editTarget = parseEditTarget(url, relativeHtmlPath, reference, sourceDocs);
      if (editTarget !== undefined) {
        if (editTarget !== "") {
          seenEditTargets.add(editTarget);
          pageEditTargets.add(editTarget);
        }
        continue;
      }

      if ((url.protocol !== "http:" && url.protocol !== "https:") || url.origin !== SITE_ORIGIN) {
        continue;
      }

      localReferenceCount += 1;
      await validateLocalReference(url, relativeHtmlPath, reference, outputFiles);
    }

    const expected = expectedEditTarget(relativeHtmlPath, sourceDocs);
    if (expected !== undefined && !pageEditTargets.has(expected)) {
      addFailure(relativeHtmlPath, `missing edit URL for ${expected}`);
    }
    for (const actual of pageEditTargets) {
      if (expected !== actual) {
        addFailure(relativeHtmlPath, `edit URL targets ${actual}; expected ${expected ?? "no source document"}`);
      }
    }
  }

  for (const sourceDoc of sourceDocs) {
    if (!seenEditTargets.has(sourceDoc)) {
      addFailure("dist", `no generated page links to edit ${sourceDoc}`);
    }
  }

  if (failures.length > 0) {
    console.error(`Generated site link check failed with ${failures.length} error${failures.length === 1 ? "" : "s"}:`);
    for (const failure of failures) {
      const reference = failure.reference
        ? ` (${failure.reference.attribute}=${JSON.stringify(failure.reference.value)})`
        : "";
      console.error(`- ${failure.source}${reference}: ${failure.message}`);
    }
    process.exitCode = 1;
    return;
  }

  console.log(
    `Checked ${htmlPaths.length} HTML files, ${localReferenceCount} local references, and ${seenEditTargets.size} edit links.`,
  );
}

await main();
