// TODO: this is inefficient but also its just for dev so don't really care.
function downloadCurrentLevel(){
    let max_len = 1024;
    for (let i=0;i<10;i++) {
        let ptr = Engine.alloc(max_len);
        let len = Engine.save_level(level, ptr, max_len);
        if (len === 0){
            Engine.drop(ptr, max_len);
            max_len *= 2;
            continue;
        }

        const buffer = new Uint8Array(Engine.memory.buffer, ptr, len);
        const blob = new Blob([buffer], {});
        const link = document.createElement('a');
        link.href = window.URL.createObjectURL(blob);
        link.download = "level.cactus";
        link.click();

        Engine.drop(ptr, max_len);
        return;
    }
    alert("Could not save level because it is bigger than " + max_len + " bytes.");
}

function loadToCurrentLevel(){
    pauseGame();
    const msg = "Loading a level will delete the current one. You may want to cancel and save it first.";
    if (!confirm(msg)) {
        resumeGame();
        return;
    }

    let input = document.createElement('input');
    input.type = 'file';

    input.onchange = (e) => {
        let file = e.target.files[0];
        file.arrayBuffer().then((res) => {
            let len = res.byteLength;
            console.log(res);
            let ptr = Engine.alloc(len);
            console.log("Loaded " + len + " bytes.");
            const data = new Uint8Array(res);
            const buffer = new Uint8Array(Engine.memory.buffer, ptr, len);
            for (let i=0;i<len;i++){
                buffer[i] = data[i];
            }
            let success = Engine.load_level(level, ptr, len);
            Engine.drop(ptr, len);
            alert(success ? "Loaded level successfully." : "Failed to load level.");
            resumeGame();
        }, (err) => {
            resumeGame();
        });
    }

    input.click();
}

function handleCanvasClick(e) {
    let x = e.offsetX;
    let y = ctx.canvas.height - e.offsetY;
    let selected = document.getElementById("objecttype").value;
    Engine.place_thing(level, x, y, parseInt(selected));
}

function handleCanvasMouseMove(e) {
    let x = e.offsetX;
    let y = ctx.canvas.height - e.offsetY;
    let selected = document.getElementById("objecttype").value;
    Engine.editor_mouse_move(x, y, parseInt(selected));
}

function initEditor(){
    document.getElementById("editor").style.display = "block";
    ctx.canvas.addEventListener("click", handleCanvasClick);
    ctx.canvas.addEventListener("mousemove", handleCanvasMouseMove);

    let objects = [
        "Delete",
        "Player",
        "Balloon",
        "Wall",
        "Box",
        "Button",
        "Wind",
        "Fan",
        "Portal"
    ];

    for (let i=0;i<objects.length;i++) {
        document.getElementById("objecttype").innerHTML += `<option value="${i}"> ${objects[i]}</option>`;
    }
}
