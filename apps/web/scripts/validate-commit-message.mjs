import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const COMMIT_TYPES = '(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)';
const HEADER_PATTERN = new RegExp(
  `^${COMMIT_TYPES}\\([a-z0-9][a-z0-9-]*\\)(!?): (?=.*[\\u3400-\\u9fff]).+$`,
  'u'
);
const BREAKING_BODY_PATTERN = /^BREAKING CHANGE:\s+\S+/u;

function getMeaningfulLines(message) {
  return message
    .replace(/\r\n/g, '\n')
    .split('\n')
    .filter((line) => !line.trimStart().startsWith('#'));
}

/**
 * 校验项目约定的提交信息。
 *
 * 普通提交只有 header；破坏性提交必须用 `!` 标记，并在 body 中说明
 * `BREAKING CHANGE:`，让提交历史可以被自动读取和人工追溯。
 */
export function validateCommitMessage(message) {
  const lines = getMeaningfulLines(message);
  const header = lines[0]?.trim() ?? '';
  const bodyLines = lines.slice(1).filter((line) => line.trim().length > 0);

  if (!HEADER_PATTERN.test(header)) {
    if (!/[㐀-鿿]/u.test(header)) {
      throw new Error('摘要必须包含中文');
    }

    throw new Error('提交信息必须符合 type(scope): 中文摘要，scope 只能使用小写字母、数字和短横线');
  }

  const isBreakingHeader = header.includes(')!:');
  const hasBreakingBody = bodyLines.some((line) => BREAKING_BODY_PATTERN.test(line.trim()));

  if (isBreakingHeader || hasBreakingBody) {
    if (!hasBreakingBody) {
      throw new Error('破坏性提交必须包含 BREAKING CHANGE body');
    }

    return;
  }

  if (bodyLines.length > 0) {
    throw new Error('普通提交不能包含 body');
  }
}

const currentFile = resolve(fileURLToPath(import.meta.url));
const invokedFile = process.argv[1] ? resolve(process.argv[1]) : '';

if (currentFile === invokedFile) {
  const messageFile = process.argv[2];

  if (!messageFile) {
    console.error('用法：node scripts/validate-commit-message.mjs <commit-message-file>');
    process.exit(1);
  }

  try {
    validateCommitMessage(readFileSync(messageFile, 'utf8'));
  } catch (error) {
    console.error(error instanceof Error ? error.message : error);
    process.exit(1);
  }
}
