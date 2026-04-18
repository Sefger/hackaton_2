<script lang="ts">
  import {onMount} from "svelte"
  import * as THREE from "three"

  let canvasContainer: HTMLDivElement
  let scrollY = 0

  onMount(() => {
    let scene: THREE.Scene, camera: THREE.PerspectiveCamera, renderer: THREE.WebGLRenderer
    let crystal: THREE.Mesh, torus: THREE.Mesh, sphere: THREE.Mesh

    function init() {
      scene = new THREE.Scene()
      camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000)
      camera.position.z = 5

      renderer = new THREE.WebGLRenderer({antialias: true, alpha: true})
      renderer.setSize(window.innerWidth, window.innerHeight)
      renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
      renderer.toneMapping = THREE.ReinhardToneMapping
      renderer.toneMappingExposure = 1.8
      canvasContainer.appendChild(renderer.domElement)

      const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
      scene.add(ambientLight)

      const pointLight = new THREE.PointLight(0x0066ff, 12)
      pointLight.position.set(5, 5, 5)
      scene.add(pointLight)

      const accentLight = new THREE.PointLight(0x4f46e5, 10)
      accentLight.position.set(-5, -5, 2)
      scene.add(accentLight)

      const crystalGeo = new THREE.IcosahedronGeometry(1.2, 0)
      const crystalMat = new THREE.MeshPhongMaterial({
        color: 0x0066ff,
        emissive: 0x0022ff,
        emissiveIntensity: 0.4,
        shininess: 150,
        flatShading: true,
        transparent: true,
        opacity: 0.85,
        side: THREE.DoubleSide,
      })
      crystal = new THREE.Mesh(crystalGeo, crystalMat)
      crystal.position.set(-2, 0, 0)
      scene.add(crystal)

      const torusGeo = new THREE.TorusGeometry(0.8, 0.25, 24, 100)
      const torusMat = new THREE.MeshStandardMaterial({
        color: 0x4f46e5,
        emissive: 0x2200ff,
        emissiveIntensity: 0.3,
        metalness: 0.9,
        roughness: 0.1,
      })
      torus = new THREE.Mesh(torusGeo, torusMat)
      torus.position.set(2, -2, -1)
      scene.add(torus)

      const sphereGeo = new THREE.SphereGeometry(1.1, 24, 24)
      const sphereMat = new THREE.MeshPhongMaterial({
        color: 0x4da6ff,
        emissive: 0x0099ff,
        emissiveIntensity: 0.5,
        wireframe: true,
      })
      sphere = new THREE.Mesh(sphereGeo, sphereMat)
      sphere.position.set(0, -5, -2)
      scene.add(sphere)

      const particlesGeo = new THREE.BufferGeometry()
      const particlesCount = 3500
      const posArray = new Float32Array(particlesCount * 3)
      for (let i = 0; i < particlesCount * 3; i++) {
        posArray[i] = (Math.random() - 0.5) * 20
      }
      particlesGeo.setAttribute("position", new THREE.BufferAttribute(posArray, 3))
      const particlesMat = new THREE.PointsMaterial({
        size: 0.015,
        color: 0x4da6ff,
        transparent: true,
        opacity: 0.6,
        blending: THREE.AdditiveBlending,
      })
      const particlesMesh = new THREE.Points(particlesGeo, particlesMat)
      scene.add(particlesMesh)

      window.addEventListener("scroll", handleScroll)
      window.addEventListener("resize", onResize)
      animate()
    }

    function handleScroll() {
      scrollY = window.scrollY
    }

    function onResize() {
      camera.aspect = window.innerWidth / window.innerHeight
      camera.updateProjectionMatrix()
      renderer.setSize(window.innerWidth, window.innerHeight)
    }

    function animate() {
      requestAnimationFrame(animate)
      const scrollHeight = document.documentElement.scrollHeight - window.innerHeight
      const scrollPercent = scrollHeight > 0 ? scrollY / scrollHeight : 0
      const isMobile = window.innerWidth < 768

      crystal.rotation.y += 0.008
      crystal.rotation.x += 0.003
      torus.rotation.z += 0.01
      sphere.rotation.y -= 0.005

      const xFactor = isMobile ? 0 : 3
      const yFactor = isMobile ? 18 : 9

      crystal.position.y = -(scrollPercent * yFactor) + (isMobile ? 0.5 : 1)
      crystal.position.x = (isMobile ? 0 : -2) + scrollPercent * xFactor

      torus.position.y = -(scrollPercent * (yFactor + 2)) + (isMobile ? -1 : 0)
      torus.position.z = -1 + scrollPercent * (isMobile ? 2 : 5)
      torus.position.x = isMobile ? 0 : 2

      sphere.position.y = -(scrollPercent * (yFactor + 4)) + (isMobile ? -2 : 2)
      sphere.position.x = 0

      camera.position.y = -(scrollPercent * (isMobile ? 4 : 3))
      camera.lookAt(0, -(scrollPercent * (isMobile ? 8 : 6)), 0)

      renderer.render(scene, camera)
    }

    init()

    return () => {
      window.removeEventListener("scroll", handleScroll)
      window.removeEventListener("resize", onResize)
      renderer.dispose()
    }
  })
</script>

<div
  bind:this={canvasContainer}
  class="fixed top-0 left-0 w-full h-screen z-[-1]"
></div>
