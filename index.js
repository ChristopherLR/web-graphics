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

  const FRAME_RATE = 60.0
  const FRAME_TIME = (1.0 / FRAME_RATE)*1000.0;
  let lastDrawTime = -1
  let elapsedTime = 0;
  let currTime = Date.now();

  let render = () => {
    window.requestAnimationFrame(render)

    currTime = Date.now()
    if (currTime - lastDrawTime >= FRAME_TIME) {
      elapsedTime = currTime - lastDrawTime;
      lastDrawTime = currTime
      
      webClient.update(currTime);
      webClient.render();
    }
  }

  render()
})
