import init, {create_puzzle} from "./pkg/crossyword.js";
init().then(() => {
    const puzzle = create_puzzle();
    console.log(puzzle);
    drawPuzzle(puzzle);
})
function drawPuzzle(puzzle) {
    const canvas = document.querySelector('#canvas');
    const ctx = canvas.getContext('2d');

    ctx.fillStyle = '#FFFFFF';
    ctx.strokeStyle = '#000000';
    for (var i = 0; i < puzzle.length; i ++) {
        var word = puzzle[i].word;

        for (var j = 0; j < word.length; j ++) {
            ctx.fillRect(25 * j, 25 * i, 25, 25);
            ctx.strokeRect(25 * j, 25 * i, 25, 25);
        }
    }
}

