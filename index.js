const rust = import('./pkg/index')

rust.then(m => m.say_hello())
  .catch(console.error)