import { describe, expect, it } from 'vitest';

import { validateCommitMessage } from '../../scripts/validate-commit-message.mjs';

describe('validateCommitMessage', () => {
  it('accepts a normal commit with a Chinese summary and no body', () => {
    expect(() => validateCommitMessage('feat(auth): 添加登录')).not.toThrow();
  });

  it('rejects a normal commit with a body', () => {
    expect(() => validateCommitMessage('feat(auth): 添加登录\n\n补充登录流程')).toThrow(
      '普通提交不能包含 body'
    );
  });

  it('rejects a summary without Chinese text', () => {
    expect(() => validateCommitMessage('feat(auth): add login')).toThrow('摘要必须包含中文');
  });

  it('requires a breaking-change body for a breaking commit', () => {
    expect(() => validateCommitMessage('feat(api)!: 调整接口')).toThrow(
      '破坏性提交必须包含 BREAKING CHANGE body'
    );
  });

  it('accepts a breaking commit with an explanatory body', () => {
    expect(() =>
      validateCommitMessage('feat(api)!: 调整接口\n\nBREAKING CHANGE: 返回结构发生变化')
    ).not.toThrow();
  });
});
