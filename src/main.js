const { invoke } = window.__TAURI__.core;

const containers = document.getElementsByClassName("container");

function symbolForPlayer(p) {
    return p === 1 ? "X" : p === 2 ? "O" : "";
}

function updateUIFromState(state) {
    for (let i = 0; i < containers.length; i++) {
        const val = state.played[i];
        containers[i].innerHTML = symbolForPlayer(val);
    }
}

for (let cont = 0; cont < containers.length; cont++) {
    containers[cont].addEventListener("click", async function () {
        // don't rely only on innerHTML; backend enforces validity
        try {
            const clicked = (this.dataset.index || "").toString();
            const result = await invoke("play", { clicked });
            const state = JSON.parse(result);
            updateUIFromState(state);
            if (state.winner) {
                alert(`Player ${state.winner} wins!`);
            } else if (state.is_draw) {
                alert("It's a draw");
            }
        } catch (e) {
            alert(e);
        }
    });
}

// On load, fetch current state and render
window.addEventListener("DOMContentLoaded", async () => {
    try {
        const result = await invoke("get_state");
        const state = JSON.parse(result);
        updateUIFromState(state);
    } catch (e) {
        // ignore; state may be default
    }

    // Reset button (optional) - if present in HTML
    const resetBtn = document.getElementById("reset");
    if (resetBtn) {
        resetBtn.addEventListener("click", async () => {
            try {
                const res = await invoke("reset");
                const state = JSON.parse(res);
                updateUIFromState(state);
            } catch (e) {
                alert(e);
            }
        });
    }
});