const rust = import('./pkg/index')

const canvas = document.getElementById('rustCanvas')

const gl = canvas.getContext('webgl2', { antialias: true })

const updateCanvas = () => {
  canvas.height = window.innerHeight
  canvas.clientHeight = window.innerHeight
  canvas.style.height = window.innerHeight

  canvas.width = window.innerWidth
  canvas.clientWidth = window.innerWidth
  canvas.style.width = window.innerWidth

  // Tell gl that the -1 to 1 scale goes from 0 to width/height
  gl.viewport(0, 0, window.innerWidth, window.innerHeight)
}

rust.then((m) => {
  if (!gl) {
    alert('Failed to init WebGl')
    return
  }

  const webClient = new m.WebClient(window.innerHeight, window.innerWidth);

  updateCanvas();

  window.addEventListener('resize', (event) => {
    if ( window.innerHeight != canvas.height || window.innerWidth != canvas.width ) {
      updateCanvas();
      webClient.update_size(window.innerHeight, window.innerWidth);
    }
  }, true)

  const FPS_THROTTLE = 1000.0 / 30.0 // millis / frames
  let lastDrawTime = -1
  let elapsedTime = 0;
  let currTime = Date.now();

  let render = () => {
    window.requestAnimationFrame(render)

    currTime = Date.now()
    if (currTime >= lastDrawTime + FPS_THROTTLE) {
      elapsedTime = currTime - lastDrawTime;
      lastDrawTime = currTime
      
      webClient.update(elapsedTime);
      webClient.render();
    }
  }

  render()
})
