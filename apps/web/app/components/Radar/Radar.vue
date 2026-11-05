<template>
  <div ref="containerRef" class="radar-container" />
</template>

<script setup lang="ts">
  import { Mesh, Program, Renderer, Triangle } from 'ogl';
  import { onBeforeUnmount, onMounted, useTemplateRef, watch } from 'vue';

  interface RadarProps {
    speed?: number;
    scale?: number;
    ringCount?: number;
    spokeCount?: number;
    ringThickness?: number;
    spokeThickness?: number;
    sweepSpeed?: number;
    sweepWidth?: number;
    sweepLobes?: number;
    color?: string;
    backgroundColor?: string;
    falloff?: number;
    brightness?: number;
    enableMouseInteraction?: boolean;
    mouseInfluence?: number;
  }

  function hexToVec3(hex: string): [number, number, number] {
    const h = hex.replace('#', '');
    return [
      parseInt(h.slice(0, 2), 16) / 255,
      parseInt(h.slice(2, 4), 16) / 255,
      parseInt(h.slice(4, 6), 16) / 255,
    ];
  }

  const vertexShader = `
attribute vec2 uv;
attribute vec2 position;
varying vec2 vUv;
void main() {
  vUv = uv;
  gl_Position = vec4(position, 0, 1);
}
`;

  const fragmentShader = `
precision highp float;

uniform float uTime;
uniform vec3 uResolution;
uniform float uSpeed;
uniform float uScale;
uniform float uRingCount;
uniform float uSpokeCount;
uniform float uRingThickness;
uniform float uSpokeThickness;
uniform float uSweepSpeed;
uniform float uSweepWidth;
uniform float uSweepLobes;
uniform vec3 uColor;
uniform vec3 uBgColor;
uniform float uFalloff;
uniform float uBrightness;
uniform vec2 uMouse;
uniform float uMouseInfluence;
uniform bool uEnableMouse;

#define TAU 6.28318530718
#define PI 3.14159265359

void main() {
  vec2 st = gl_FragCoord.xy / uResolution.xy;
  st = st * 2.0 - 1.0;
  st.x *= uResolution.x / uResolution.y;

  if (uEnableMouse) {
    vec2 mShift = (uMouse * 2.0 - 1.0);
    mShift.x *= uResolution.x / uResolution.y;
    st -= mShift * uMouseInfluence;
  }

  st *= uScale;

  float dist = length(st);
  float theta = atan(st.y, st.x);
  float t = uTime * uSpeed;

  float ringPhase = dist * uRingCount - t;
  float ringDist = abs(fract(ringPhase) - 0.5);
  float ringGlow = 1.0 - smoothstep(0.0, uRingThickness, ringDist);

  float spokeAngle = abs(fract(theta * uSpokeCount / TAU + 0.5) - 0.5) * TAU / uSpokeCount;
  float arcDist = spokeAngle * dist;
  float spokeGlow = (1.0 - smoothstep(0.0, uSpokeThickness, arcDist)) * smoothstep(0.0, 0.1, dist);

  float sweepPhase = t * uSweepSpeed;
  float sweepBeam = pow(max(0.5 * sin(uSweepLobes * theta + sweepPhase) + 0.5, 0.0), uSweepWidth);

  float fade = smoothstep(1.05, 0.85, dist) * pow(max(1.0 - dist, 0.0), uFalloff);

  float intensity = max((ringGlow + spokeGlow + sweepBeam) * fade * uBrightness, 0.0);
  vec3 col = uColor * intensity + uBgColor;

  float alpha = clamp(length(col), 0.0, 1.0);
  gl_FragColor = vec4(col, alpha);
}
`;

  const props = withDefaults(defineProps<RadarProps>(), {
    speed: 0.6,
    scale: 0.55,
    ringCount: 8.0,
    spokeCount: 8.0,
    ringThickness: 0.04,
    spokeThickness: 0.008,
    sweepSpeed: 0.8,
    sweepWidth: 2.5,
    sweepLobes: 1.0,
    color: '#B8D4E3',
    backgroundColor: '#F8F9FC',
    falloff: 2.2,
    brightness: 0.55,
    enableMouseInteraction: false,
    mouseInfluence: 0.05,
  });

  const containerRef = useTemplateRef<HTMLDivElement>('containerRef');

  let cleanup: (() => void) | null = null;
  const setup = () => {
    if (!containerRef.value) return;
    const container = containerRef.value;
    const renderer = new Renderer({ alpha: true, premultipliedAlpha: false });
    const gl = renderer.gl;
    gl.clearColor(0, 0, 0, 0);

    const currentMouse: [number, number] = [0.5, 0.5];
    let targetMouse: [number, number] = [0.5, 0.5];

    function handleMouseMove(e: MouseEvent) {
      const rect = gl.canvas.getBoundingClientRect();
      targetMouse = [
        (e.clientX - rect.left) / rect.width,
        1.0 - (e.clientY - rect.top) / rect.height,
      ];
    }

    function handleMouseLeave() {
      targetMouse = [0.5, 0.5];
    }

    renderer.setSize(container.offsetWidth, container.offsetHeight);

    const geometry = new Triangle(gl);
    const program = new Program(gl, {
      vertex: vertexShader,
      fragment: fragmentShader,
      uniforms: {
        uTime: { value: 0 },
        uResolution: {
          value: [gl.canvas.width, gl.canvas.height, gl.canvas.width / gl.canvas.height],
        },
        uSpeed: { value: props.speed },
        uScale: { value: props.scale },
        uRingCount: { value: props.ringCount },
        uSpokeCount: { value: props.spokeCount },
        uRingThickness: { value: props.ringThickness },
        uSpokeThickness: { value: props.spokeThickness },
        uSweepSpeed: { value: props.sweepSpeed },
        uSweepWidth: { value: props.sweepWidth },
        uSweepLobes: { value: props.sweepLobes },
        uColor: { value: hexToVec3(props.color) },
        uBgColor: { value: hexToVec3(props.backgroundColor) },
        uFalloff: { value: props.falloff },
        uBrightness: { value: props.brightness },
        uMouse: { value: new Float32Array([0.5, 0.5]) },
        uMouseInfluence: { value: props.mouseInfluence },
        uEnableMouse: { value: props.enableMouseInteraction },
      },
    });

    function resize() {
      renderer.setSize(container.offsetWidth, container.offsetHeight);
      program.uniforms.uResolution.value = [
        gl.canvas.width,
        gl.canvas.height,
        gl.canvas.width / gl.canvas.height,
      ];
    }
    window.addEventListener('resize', resize);

    const mesh = new Mesh(gl, { geometry, program });
    container.appendChild(gl.canvas);

    if (props.enableMouseInteraction) {
      gl.canvas.addEventListener('mousemove', handleMouseMove);
      gl.canvas.addEventListener('mouseleave', handleMouseLeave);
    }

    let animationFrameId: number;

    function update(time: number) {
      animationFrameId = requestAnimationFrame(update);
      program.uniforms.uTime.value = time * 0.001;

      const uMouse = program.uniforms.uMouse.value;
      if (props.enableMouseInteraction) {
        currentMouse[0] += 0.05 * (targetMouse[0] - currentMouse[0]);
        currentMouse[1] += 0.05 * (targetMouse[1] - currentMouse[1]);
        if (uMouse) {
          uMouse[0] = currentMouse[0];
          uMouse[1] = currentMouse[1];
        }
      } else if (uMouse) {
        uMouse[0] = 0.5;
        uMouse[1] = 0.5;
      }

      renderer.render({ scene: mesh });
    }
    animationFrameId = requestAnimationFrame(update);

    cleanup = () => {
      cancelAnimationFrame(animationFrameId);
      window.removeEventListener('resize', resize);
      if (props.enableMouseInteraction) {
        gl.canvas.removeEventListener('mousemove', handleMouseMove);
        gl.canvas.removeEventListener('mouseleave', handleMouseLeave);
      }
      container.removeChild(gl.canvas);
      gl.getExtension('WEBGL_lose_context')?.loseContext();
    };
  };

  onMounted(() => {
    setup();
  });

  onBeforeUnmount(() => {
    cleanup?.();
  });

  watch(
    () => [
      props.speed,
      props.scale,
      props.ringCount,
      props.spokeCount,
      props.ringThickness,
      props.spokeThickness,
      props.sweepSpeed,
      props.sweepWidth,
      props.sweepLobes,
      props.color,
      props.backgroundColor,
      props.falloff,
      props.brightness,
      props.enableMouseInteraction,
      props.mouseInfluence,
    ],
    () => {
      cleanup?.();
      setup();
    }
  );
</script>

<style scoped>
  .radar-container {
    width: 100%;
    height: 100%;
  }
</style>
