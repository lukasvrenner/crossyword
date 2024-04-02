import init, {create_puzzle, Orientation} from "./wasm/crossyword.js";
init().then(() => {
    const puzzle = create_puzzle();
    drawPuzzle(puzzle);
    drawClues(puzzle);
    document.getElementById("answers-toggle").onclick = () => {
        if (document.getElementById("answers-toggle").innerHTML == "Show Answers") { 
            showAnswers(puzzle);
        }
        else {
            hideAnswers(puzzle);
        }
    }
})

const canvas = document.querySelector('#canvas');
const ctx = canvas.getContext('2d');
const boxSize = canvas.width / 20;
ctx.font = "bold 9pt monospace";

function drawPuzzle(puzzle) {
    ctx.strokeStyle = '#000000';
    for (var i = 0; i < puzzle.length; i ++) {
        const word = puzzle[i];
        ctx.fillStyle = '#FFFFFF';

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
        ctx.fillStyle = "#000000";
        ctx.fillText(i + 1, word.xpos * boxSize, (word.ypos) * boxSize + 10);
    }
}

function drawClues(puzzle) {
    for (var i = 0; i < puzzle.length; i ++) {
        const word = puzzle[i];
        if (word.orientation == Orientation.Vertical) {
            document.getElementById("vertical-clues").innerHTML
                += (i + 1) + ". " + word.clue + "<br>";
        } else {
            document.getElementById("horizontal-clues").innerHTML
                += (i + 1) + ". " + word.clue + "<br>";
        }
    }
}

function showAnswers(puzzle) {
    ctx.fillStyle = "#000000";
    document.getElementById("answers-toggle").innerHTML = "Hide Answers"
    for (const word of puzzle) {
        if (word.orientation == Orientation.Horizontal) {
            for (var i = 0; i < word.word.length; i ++) {
                ctx.fillText(
                    word.word[i], 
                    (word.xpos + 0.4 + i) * boxSize, 
                    (word.ypos + 0.6) * boxSize
                );
            }
        } else {
            for (var i = 0; i < word.word.length; i ++) {
                ctx.fillText(
                    word.word[i], 
                    (word.xpos + 0.4) * boxSize,
                    (word.ypos + 0.6 + i) * boxSize
                );
            }
        }
    }
}

function hideAnswers(puzzle) {
    document.getElementById("answers-toggle").innerHTML = "Show Answers"
    drawPuzzle(puzzle);
}
