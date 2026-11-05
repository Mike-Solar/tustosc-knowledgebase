import { describe, expect, it } from 'vitest';

import { cn } from '../../app/lib/utils';

describe('cn', () => {
  it('merges conditional and conflicting utility classes', () => {
    const includeHidden = false;

    expect(cn('px-2', includeHidden ? 'hidden' : undefined, 'px-4', 'text-foreground')).toBe(
      'px-4 text-foreground'
    );
  });
});
