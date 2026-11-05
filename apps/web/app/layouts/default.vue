<script setup lang="ts">
  import type { CardNavItem } from '~/components/CardNav/CardNav.vue';
  import Aurora from '~/components/Aurora/Aurora.vue';
  import CardNav from '~/components/CardNav/CardNav.vue';
  import SpecularButton from '~/components/SpecularButton/SpecularButton.vue';

  const router = useRouter();
  const route = useRoute();
  const isHome = computed(() => route.path === '/');

  useHead(() => ({
    bodyAttrs: {
      style: isHome.value ? 'background: transparent' : undefined,
    },
  }));

  const cardNavItems: CardNavItem[] = [
    {
      label: '关于协会',
      bgColor: '#FFF0E9',
      textColor: '#8B3A3A',
      links: [
        { label: '示例', ariaLabel: '功能示例占位' },
        { label: '示例', ariaLabel: '功能示例占位' },
      ],
    },
    {
      label: '知识空间',
      bgColor: '#F0E8F7',
      textColor: '#5A3D6E',
      links: [
        { label: '进入工作台', href: '/workspace', ariaLabel: '进入知识工作台' },
        { label: '示例', ariaLabel: '功能示例占位' },
      ],
    },
    {
      label: '联系协会',
      bgColor: '#EEF1F5',
      textColor: '#3A4A5A',
      links: [
        { label: '示例', ariaLabel: '功能示例占位' },
        { label: '示例', ariaLabel: '功能示例占位' },
      ],
    },
  ];

  const goToWorkspace = () => void router.push('/workspace');
</script>

<template>
  <div class="site-shell">
    <div v-if="isHome" class="home-aurora-backdrop" aria-hidden="true">
      <Aurora />
    </div>

    <header class="site-header">
      <CardNav
        class-name="site-card-nav"
        :logo="'/icon/favicon.svg'"
        logo-alt="开源协会知识库图标"
        :items="cardNavItems"
        base-color="#ffffff"
        menu-color="#171b25"
      >
        <template #brand>
          <NuxtLink class="card-nav-brand" to="/" aria-label="返回知识库首页">
            <span
              class="card-nav-brand-mark"
              role="img"
              aria-label="开源协会知识库图标"
              style="background-image: url('/icon/favicon.svg')"
            />
            <strong>开源协会知识库</strong>
          </NuxtLink>
        </template>

        <template #action>
          <div class="site-card-nav-action">
            <SpecularButton
              size="sm"
              :radius="12"
              tint="#ffffff"
              :tint-opacity="0"
              text-color="#171b25"
              line-color="#e86f4f"
              base-color="#ffffff"
              :intensity="0.5"
              :shine-size="12"
              :shine-fade="35"
              :thickness="1.2"
              :speed="0.35"
              :proximity="250"
              :auto-animate="false"
              aria-label="进入工作台"
              @click="goToWorkspace"
            >
              进入工作台
            </SpecularButton>
          </div>
        </template>
      </CardNav>
    </header>

    <main>
      <slot />
    </main>

    <footer class="site-footer">
      <span>天津科技大学开放原子开源协会</span>
      <span>让知识在连接中持续生长</span>
    </footer>
  </div>
</template>
