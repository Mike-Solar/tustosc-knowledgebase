import type { ClassValue } from 'clsx';
import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

/**
 * shadcn 组件通过这个边界合并条件类名和 Tailwind 冲突类名。
 *
 * UI 组件只依赖 class 合并，不把样式判断扩散到页面或业务层。
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
