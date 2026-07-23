const { invoke } = window.__TAURI__.core;

const containers = document.getElementsByClassName('container');
let showMessage = false;

function updateUIFromState(squares) {
  for (let i = 0; i < containers.length; i++) {
    containers[i].innerHTML = squares[i];
  }
}

for (let cont = 0; cont < containers.length; cont++) {
  containers[cont].addEventListener('click', async function () {
    if (showMessage) return;

    try {
      const clickedSquare = parseInt(this.dataset.index);
      let state = await invoke('apply_move', { clickedSquare });
      state = JSON.parse(state);

      updateUIFromState(state.squares);

      const msgEl = document.getElementById('message');

      if (state.winner != null) {
        msgEl.innerHTML = `o ${state.winner} ganhou!`;
        showMessage = true;
        setTimeout(() => {
          updateUIFromState(['', '', '', '', '', '', '', '', '']);
          msgEl.innerHTML = '';
          showMessage = false;
        }, 3000);
      } else if (state.is_draw) {
        msgEl.innerHTML = 'Empate';
        showMessage = true;
        setTimeout(() => {
          updateUIFromState(['', '', '', '', '', '', '', '', '']);
          msgEl.innerHTML = '';
          showMessage = false;
        }, 3000);
      } else if (state.err) {
        msgEl.innerHTML = state.err;
        setTimeout(() => {
          msgEl.innerHTML = '';
        }, 3000);
      }
    } catch (e) {
      alert(e);
    }
  });
}

document.addEventListener('DOMContentLoaded', async () => {
  try {
    let state = await invoke('get_state');
    state = JSON.parse(state);
    updateUIFromState(state.squares);
  } catch (e) {
    alert(e);
  }
});

document.getElementById('reset').addEventListener('click', async () => {
  if (showMessage) return;

  try {
    await invoke('reset');
    updateUIFromState(['', '', '', '', '', '', '', '', '']);
  } catch (e) {
    alert(e);
  }
});
