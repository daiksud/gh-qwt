// @ts-check
import { defineConfig } from 'astro/config';
import { satteri } from '@astrojs/markdown-satteri';
import starlight from '@astrojs/starlight';
import mermaid from 'astro-mermaid';
import { docsCleanup } from './src/plugins/docs-cleanup.ts';
import { githubAlerts } from './src/plugins/github-alerts.ts';

const adrPages = [
  '0001-record-architecture-decisions',
  '0002-distribute-as-gh-cli-extension',
  '0003-language-rust-precompiled-binary',
  '0004-bare-repo-plus-per-branch-worktree-layout',
  '0005-path-layout-without-host-segment',
  '0006-root-configuration-qwt-root',
  '0007-command-set-v1',
  '0008-shell-integration-for-cd',
  '0009-default-branch-detection-strategy',
  '0010-release-and-distribution',
  '0011-conventional-commits-and-release-notes',
  '0012-flat-queryable-list-output',
  '0013-remove-rm-unification-and-prune-redefinition',
].map((name) => `development/adr/${name}/index`);

export default defineConfig({
  site: 'https://daiksud.github.io',
  base: '/gh-qwt',
  trailingSlash: 'always',
  vite: {
    build: {
      // Mermaid is loaded only on diagram pages; its core chunk is about 650 kB.
      chunkSizeWarningLimit: 700,
    },
  },
  integrations: [
    mermaid(),
    starlight({
      title: 'gh-qwt',
      description:
        'Documentation for gh qwt, a GitHub CLI extension for working with one Git worktree per branch.',
      logo: {
        src: './src/assets/logo.svg',
      },
      favicon: '/favicon.svg',
      social: [
        {
          icon: 'github',
          label: 'GitHub',
          href: 'https://github.com/daiksud/gh-qwt',
        },
      ],
      editLink: {
        // The glob loader records external sources as `../docs/...`. Keeping
        // `.pages/` in the base makes URL normalization resolve to `main/docs/...`.
        baseUrl: 'https://github.com/daiksud/gh-qwt/edit/main/.pages/',
      },
      customCss: ['./src/styles/custom.css', './src/styles/github-alerts.css'],
      markdown: {
        processedDirs: ['../docs/'],
      },
      tableOfContents: {
        minHeadingLevel: 2,
        maxHeadingLevel: 3,
      },
      sidebar: [
        { label: 'Home', link: '/' },
        {
          label: 'Guides',
          items: [
            'guides/index',
            'guides/getting-started/index',
            'guides/working-with-worktrees/index',
            'guides/configuration/index',
            'guides/shell-integration/index',
          ],
        },
        {
          label: 'References',
          items: [
            'references/index',
            'references/cli/index',
            'references/configuration/index',
            'references/directory-layout/index',
            'references/glossary/index',
          ],
        },
        {
          label: 'Development',
          collapsed: true,
          items: [
            'development/index',
            'development/architecture/index',
            'development/specification/index',
            'development/building-and-releasing/index',
            'development/contributing/index',
            'development/testing/index',
            {
              label: 'Architecture Decision Records',
              collapsed: true,
              items: [
                'development/adr/index',
                ...adrPages,
                'development/adr/template/index',
              ],
            },
          ],
        },
      ],
    }),
  ],
  markdown: {
    processor: satteri({
      mdastPlugins: [docsCleanup, githubAlerts()],
    }),
  },
});
