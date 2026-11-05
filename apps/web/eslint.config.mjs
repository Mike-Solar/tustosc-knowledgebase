import { createConfigForNuxt } from '@nuxt/eslint-config';

export default [
  ...(await createConfigForNuxt()),
  {
    // shadcn 和 Vue Bits 组件由上游维护，项目只检查自己的包装层。
    ignores: [
      'app/components/ui/**',
      'app/components/CardNav/**',
      'app/components/SpecularButton/**',
    ],
  },
];
