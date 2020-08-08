import('../pkg').then(rust => {
  const state = {
    isRunning: false,
    timeout: 100,
    width: 100,
    height: 50,
    aliveOdds: 0.2,
  };

  const createUniverse = () => {
    let universe = rust.Universe.new(state.width, state.height, state.aliveOdds);
    universe.render();
    return universe;
  };


  let universe = createUniverse();
  const cellSize = universe.get_cell_size();

  const renderLoop = () => {
    universe.render();

    if (state.isRunning === true) {
      setTimeout(() => {
        console.time('TICK');
        universe.tick();
        console.timeEnd('TICK');
        startLoop();
      }, state.timeout);
    }
  };

  const startLoop = () => requestAnimationFrame(renderLoop);

  document.getElementById('canvas').addEventListener('click', e => {
    universe.click(Math.floor(e.layerX / cellSize), Math.floor(e.layerY / cellSize));
    universe.render();
  });
  document.getElementById('run-pause').addEventListener('click', () => {
    state.isRunning = !state.isRunning;
    let label = 'Run';
    if (state.isRunning === true) {
      startLoop();
      label = 'Pause';
    }
    document.getElementById('run-pause').innerText = label;
  });
  document.getElementById('reset').addEventListener('click', () => {
    state.isRunning = false;
    universe = createUniverse();
  });
  document.getElementById('height').addEventListener('change', e => {
    state.height = parseInt(e.target.value);
    universe = createUniverse();
  });
  document.getElementById('width').addEventListener('change', e => {
    state.width = parseInt(e.target.value);
    universe = createUniverse();
  });
  document.getElementById('alive-odds').addEventListener('change', e => {
    state.aliveOdds = parseFloat(e.target.value);
    universe = createUniverse();
  });
  document.getElementById('timeout').addEventListener('change', e => {
    state.timeout = parseInt(e.target.value);
  });

  document.getElementById('height').value = state.height;
  document.getElementById('width').value = state.width;
  document.getElementById('alive-odds').value = state.aliveOdds;
  document.getElementById('timeout').value = state.timeout;
  startLoop();
});
