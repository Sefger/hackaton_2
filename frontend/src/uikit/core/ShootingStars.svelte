<script lang="ts">
  const starCount = 20
  let stars = $state(
    Array.from({length: starCount}, (_, i) => ({
      id: i,
      top: Math.random() * 80 + "%",
      left: Math.random() * 100 + "%",
      delay: Math.random() * 20 + "s",
      duration: Math.random() * 4 + 3 + "s",
      color: ["#3b82f6", "#8b5cf6", "#fff", "#06b6d4"][Math.floor(Math.random() * 4)],
      scale: Math.random() * 0.5 + 0.5,
    }))
  )
</script>

<div class="stars-container pointer-events-none fixed inset-0 overflow-hidden z-0 select-none">
  {#each stars as star}
    <div
      class="absolute w-[2px] h-[2px] opacity-0 animate-shoot before:content-[''] before:absolute before:top-1/2 before:-translate-y-1/2 before:w-[150px] before:h-[1.5px] before:bg-gradient-to-r before:from-[var(--star-color)] before:to-transparent"
      style:top={star.top}
      style:left={star.left}
      style:animation-delay={star.delay}
      style:animation-duration={star.duration}
      style:background-color={star.color}
      style:border-radius="50%"
      style:--star-color={star.color}
      style:--star-scale={star.scale}
      style:box-shadow="0 0 8px {star.color}"
    ></div>
  {/each}
</div>
