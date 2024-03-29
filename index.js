import init, {create_puzzle, Orientation} from "./pkg/crossyword.js";
init().then(() => {
    const puzzle = create_puzzle();
    drawPuzzle(puzzle);
    drawClues(puzzle);
})

const canvas = document.querySelector('#canvas');
const ctx = canvas.getContext('2d');

const boxSize = canvas.width / 20;
function drawPuzzle(puzzle) {

    ctx.strokeStyle = '#000000';
    ctx.fillStyle = '#FFFFFF';
    for (var i = 0; i < puzzle.length; i ++) {
        var word = puzzle[i];

        if (word.orientation == Orientation.Horizontal) {
            for (var j = 0; j < word.word.length; j ++) {
                ctx.fillRect(
                    (word.xpos + j) * boxSize,
                    boxSize * word.ypos, 
                    boxSize, boxSize);

                ctx.strokeRect(
                    (word.xpos + j) * boxSize,
                    boxSize * word.ypos, 
                    boxSize, boxSize);

            }
        } else {
            for (var j = 0; j < word.word.length; j ++) {
                ctx.fillRect(
                    word.xpos * boxSize, 
                    boxSize * (word.ypos + j), 
                    boxSize, boxSize);

                ctx.strokeRect(word.xpos * boxSize,
                    boxSize * (word.ypos + j),
                    boxSize, boxSize);
            }
        }
        
    }
}

function drawClues(puzzle) {
    ctx.fillStyle = "#000000";
    for (var i = 0; i < puzzle.length; i ++) {
        var word = puzzle[i];
        ctx.fillStyle = "#000000";
        ctx.fillText(i + 1, word.xpos * boxSize, (word.ypos + 0.5) * boxSize);
        if (word.orientation == Orientation.Vertical) {
            document.getElementById("vertical-clues").innerHTML
                += (i + 1) + ". " + word.clue + "<br>";
        } else {
            document.getElementById("horizontal-clues").innerHTML
                += (i + 1) + ". " + word.clue + "<br>";
        }
    }
}

