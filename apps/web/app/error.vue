<script setup lang="ts">
  import type { NuxtError } from '#app';

  import Radar from '~/components/Radar/Radar.vue';
  import SpecularButton from '~/components/SpecularButton/SpecularButton.vue';

  defineProps<{
    error: NuxtError;
  }>();

  const handleClearError = () => clearError({ redirect: '/' });
</script>

<template>
  <div class="error-page" role="main" aria-labelledby="error-title">
    <div class="error-radar" aria-hidden="true">
      <Radar :speed="0.5" :brightness="0.45" />
    </div>

    <div class="error-content">
      <p v-if="error.statusCode" class="error-code">{{ error.statusCode }}</p>
      <h1 id="error-title" class="error-title">该页面为示例页或页面不存在</h1>
      <p class="error-hint">点击下方回到首页</p>
      <SpecularButton
        size="md"
        :radius="16"
        tint="#ffffff"
        :tint-opacity="0"
        text-color="#171b25"
        line-color="#e86f4f"
        base-color="#525252"
        :intensity="1.2"
        :shine-size="12"
        :shine-fade="35"
        :thickness="1.2"
        :speed="0.35"
        :proximity="250"
        :auto-animate="false"
        aria-label="回到首页"
        @click="handleClearError"
      >
        回到首页
      </SpecularButton>
    </div>
  </div>
</template>

<style scoped>
  .error-page {
    position: relative;
    display: grid;
    min-height: 100vh;
    place-items: center;
    overflow: hidden;
    background: #f7f8fb;
  }

  .error-radar {
    position: absolute;
    inset: 0;
    z-index: 0;
    pointer-events: none;
  }

  .error-content {
    position: relative;
    z-index: 1;
    max-width: 480px;
    padding: 48px 32px;
    text-align: center;
  }

  .error-code {
    margin: 0 0 16px;
    color: #b8d4e3;
    font-size: clamp(64px, 12vw, 120px);
    font-weight: 650;
    letter-spacing: -0.06em;
    line-height: 1;
  }

  .error-title {
    margin: 0;
    color: #171b25;
    font-size: clamp(20px, 4vw, 28px);
    font-weight: 540;
    letter-spacing: -0.02em;
    line-height: 1.4;
  }

  .error-hint {
    margin: 16px 0 32px;
    color: #6f7786;
    font-size: 15px;
  }
</style>
