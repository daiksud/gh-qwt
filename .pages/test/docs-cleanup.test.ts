import { describe, expect, test } from 'bun:test';
import { markdownToHtml } from 'satteri';
import { docsCleanup } from '../src/plugins/docs-cleanup';

describe('docs cleanup plugin', () => {
  test('removes the source H1 and manual table of contents only', () => {
    const { html } = markdownToHtml(
      [
        '# Page title',
        '',
        'Introduction.',
        '',
        '## Table of contents',
        '',
        '- [First](#first)',
        '- [Second](#second)',
        '',
        '## First',
        '',
        'Body.',
      ].join('\n'),
      { mdastPlugins: [docsCleanup] },
    );

    expect(html).not.toContain('<h1>');
    expect(html).not.toContain('Table of contents');
    expect(html).not.toContain('href="#first"');
    expect(html).toContain('<p>Introduction.</p>');
    expect(html).toContain('<h2>First</h2>');
  });

  test('does not remove a contents heading without a following list', () => {
    const { html } = markdownToHtml('# Title\n\n## Contents\n\nA prose section.', {
      mdastPlugins: [docsCleanup],
    });

    expect(html).toContain('<h2>Contents</h2>');
    expect(html).toContain('<p>A prose section.</p>');
  });
});
