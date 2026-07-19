import { defineMdastPlugin } from 'satteri';

const tableOfContentsHeading = /^(?:contents|table of contents)$/i;

/**
 * Removes source-only navigation that is useful on GitHub but duplicated by
 * Starlight. This runs only while building the site; source Markdown is kept
 * unchanged.
 */
export function docsCleanup() {
  let removedLeadingHeading = false;

  return defineMdastPlugin({
    name: 'gh-qwt-docs-cleanup',
    heading(node, context) {
      const parent = context.parent(node);
      if (parent?.type !== 'root') return;

      if (!removedLeadingHeading && node.depth === 1) {
        removedLeadingHeading = true;
        context.removeNode(node);
        return;
      }

      if (node.depth !== 2 || !tableOfContentsHeading.test(context.textContent(node).trim())) {
        return;
      }

      const index = context.indexOf(node);
      const nextNode = index === undefined ? undefined : parent.children[index + 1];
      if (nextNode?.type !== 'list') return;

      context.removeNode(nextNode);
      context.removeNode(node);
    },
  });
}
