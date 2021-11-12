
function draw(image){
    const canvas = document.getElementById("main_canvas");
    const context = canvas.getContext("2d");
    context.fillStyle = "blue";
    context.fillRect(0, 0, 50, 50);

}
async function main(){
    const lib = await import("../pkg/index.js").catch(console.error);
    const image = new lib.Image();
    draw(image);
}
main();
