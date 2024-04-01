import init, {create_puzzle, Orientation} from "./wasm/crossyword.js";
init().then(() => {
    const puzzle = create_puzzle();
    drawPuzzle(puzzle);
    drawClues(puzzle);
    
    const showAnswersButton = document.getElementById("show-answers");
    showAnswersButton.onclick = () => {

        drawPuzzle(puzzle);
        drawClues(puzzle);

        if (showAnswersButton.innerHTML == "Show Answers") {
            showAnswersButton.innerHTML = "Hide Answers";
            showAnswers(puzzle);
        } else {
            showAnswersButton.innerHTML = "Show Answers";
        }
    }
})

const canvas = document.querySelector('#canvas');
const ctx = canvas.getContext('2d');
const boxSize = canvas.width / 20;
ctx.font = "bold 9pt monospace";

function drawPuzzle(puzzle) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.strokeStyle = '#000000';
    ctx.fillStyle = '#FFFFFF';
    for (const word of puzzle) {

        if (word.orientation == Orientation.Horizontal) {
            for (var i = 0; i < word.word.length; i ++) {
                ctx.fillRect(
                    (word.xpos + i) * boxSize,
                    boxSize * word.ypos, 
                    boxSize, boxSize);

                ctx.strokeRect(
                    (word.xpos + i) * boxSize,
                    boxSize * word.ypos, 
                    boxSize, boxSize);

            }
        } else {
            for (var i = 0; i < word.word.length; i ++) {
                ctx.fillRect(
                    word.xpos * boxSize, 
                    boxSize * (word.ypos + i), 
                    boxSize, boxSize);

                ctx.strokeRect(word.xpos * boxSize,
                    boxSize * (word.ypos + i),
                    boxSize, boxSize);
            }
        }
        
    }
}

function drawClues(puzzle) {
    ctx.fillStyle = "#000000";

    document.getElementById("vertical-clues").innerHTML = "";
    document.getElementById("horizontal-clues").innerHTML = "";

    for (var i = 0; i < puzzle.length; i ++) {
        const word = puzzle[i];
        ctx.fillStyle = "#000000";
        ctx.fillText(i + 1, word.xpos * boxSize, (word.ypos) * boxSize + 10);
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

