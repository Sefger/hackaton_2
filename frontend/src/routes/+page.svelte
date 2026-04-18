<script lang="ts">
  import Section from "$components/Landing/Section.svelte"
  import TechBadge from "$components/Landing/TechBadge.svelte"
  import ThreeScene from "$components/Landing/ThreeScene.svelte"
</script>

<ThreeScene />

<main class="relative z-10">
  <Section className="text-center md:text-left">
    <div class=" mb-6 flex flex-wrap justify-center md:justify-start gap-2">
      <TechBadge text="VK Tech Case" />
      <TechBadge text="Svelte" />
      <TechBadge text="Rust" />
    </div>
    <h1
      class="text-4xl md:text-7xl font-black mb-6 leading-[1.1] bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-indigo-500 tracking-tighter"
    >
      Enterprise <br class="hidden md:block" />Search Engine
    </h1>
    <p class="text-gray-300 text-lg md:text-2xl font-light leading-relaxed max-w-xl mx-auto md:mx-0">
      Прототип поискового движка для корпоративного цифрового рабочего пространства. Быстро. Масштабируемо. Надежно.
    </p>
  </Section>

  <Section
    align="right"
    className="text-center md:text-right"
  >
    <h2 class="text-3xl md:text-5xl font-bold mb-6 text-blue-400 tracking-tight">Производительность Rust</h2>
    <p class="text-gray-400 text-base md:text-xl leading-relaxed">
      Бэкенд на Rust обеспечивает невероятную скорость индексации документов и мгновенную обработку сложных поисковых
      запросов в реальном времени, минимизируя нагрузку на инфраструктуру.
    </p>
  </Section>

  <Section className="text-center md:text-left">
    <h2 class="text-3xl md:text-5xl font-bold mb-6 text-indigo-400 tracking-tight">Реактивность Svelte</h2>
    <p class="text-gray-400 text-base md:text-xl leading-relaxed">
      Легковесный фронтенд на Svelte гарантирует моментальный отклик интерфейса. Поиск по мере ввода, умные фильтры и
      навигация без задержек — стандарт современного рабочего места.
    </p>
  </Section>

  <Section align="center">
    <h2 class="text-3xl md:text-5xl font-bold mb-10 text-white tracking-tight">Готовы протестировать поиск?</h2>
    <div class="flex justify-center">
      <a
        href="/search"
        class="w-full px-16 py-5 bg-blue-600 text-white font-black rounded-2xl hover:bg-blue-500 transition-all transform hover:scale-105 shadow-[0_0_30px_rgba(37,99,235,0.3)] text-center uppercase tracking-widest text-sm"
      >
        Вперед
      </a>
    </div>
  </Section>
</main>
