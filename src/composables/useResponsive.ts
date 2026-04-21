import { ref, onMounted, onUnmounted } from 'vue';

export function useResponsive() {
  const windowWidth = ref(window.innerWidth);
  
  // Logical breakpoints
  const isMobile = ref(windowWidth.value < 768);
  const isTablet = ref(windowWidth.value >= 768 && windowWidth.value < 1024);
  const isDesktop = ref(windowWidth.value >= 1024);

  // Dynamic Font Size Refs (Scaling down automatically on mobile)
  const fontSm = ref(isMobile.value ? 'text-[10px]' : 'text-sm');
  const fontBase = ref(isMobile.value ? 'text-sm' : 'text-base');
  const fontLg = ref(isMobile.value ? 'text-base' : 'text-lg');
  const fontXl = ref(isMobile.value ? 'text-lg' : 'text-xl');
  const font2xl = ref(isMobile.value ? 'text-xl' : 'text-2xl');

  function handleResize() {
    windowWidth.value = window.innerWidth;
    isMobile.value = windowWidth.value < 768;
    isTablet.value = windowWidth.value >= 768 && windowWidth.value < 1024;
    isDesktop.value = windowWidth.value >= 1024;

    // Update fonts based on JS tracking
    fontSm.value = isMobile.value ? 'text-[10px]' : 'text-sm';
    fontBase.value = isMobile.value ? 'text-sm' : 'text-base';
    fontLg.value = isMobile.value ? 'text-base' : 'text-lg';
    fontXl.value = isMobile.value ? 'text-lg' : 'text-xl';
    font2xl.value = isMobile.value ? 'text-xl' : 'text-2xl';
  }

  onMounted(() => {
    window.addEventListener('resize', handleResize);
  });

  onUnmounted(() => {
    window.removeEventListener('resize', handleResize);
  });

  return { 
    windowWidth, isMobile, isTablet, isDesktop, 
    fontSm, fontBase, fontLg, fontXl, font2xl 
  };
}