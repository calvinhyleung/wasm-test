
function draw(image){
    const canvas = document.getElementById("main_canvas");
    const context = canvas.getContext("2d");
    // context.fillStyle = "blue";
    // context.fillRect(0, 0, 50, 50);

    context.strokeStyle = "black"
    context.lineWidth  = 1;

    const width = image.width();
    const height = image.height();
    const cell_size = image.cell_size();
    const cells = image.cells();
    for (let x = 0; x < width; x++){
        for (let y = 0; y < height; y++){
            const index = (y * width + x) * 3;
            const color = `rgb(${cells[index + 0]}, ${cells[index + 1]}, ${cells[index + 2]})`;
            context.fillStyle = color;
            context.fillRect(x * cell_size, y * cell_size, cell_size, cell_size);
        }
    }
    for (let x = 0; x <= width; x++) {
        context.beginPath();
        context.moveTo(x * cell_size + .5, 0);
        context.lineTo(x * cell_size, height * cell_size);
        context.stroke();

    }
    for (let y = 0; y <= height; y++) {
        context.beginPath();
        context.moveTo(0, y * cell_size + .5);
        context.lineTo(width * cell_size, y * cell_size);
        context.stroke();
    }
}
function setupCanvas(image) {
    const canvas = document.getElementById("main_canvas");
    const cell_size = image.cell_size();;
    canvas.addEventListener("mousedown", (event) => {
        const rect = canvas.getBoundingClientRect();
        let x = event.clientX - rect.left;
        let y = event.clientY - rect.top;
        x = Math.floor(x/cell_size);
        y = Math.floor(y/cell_size);
        image.brush(x,y,[200,255,200]);
        draw(image);
    });
}
async function main(){
    const lib = await import("../pkg/index.js").catch(console.error);
    const image = new lib.Image(10,10,50);
    draw(image);
    setupCanvas(image);
    
}
main();
