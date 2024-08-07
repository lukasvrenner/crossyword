import init, {create_puzzle, Orientation} from "./wasm/crossyword.js";
init().then(() => {
    const puzzle = create_puzzle();
    let guessedWords = Array(puzzle.length).fill("");

    drawPuzzle(puzzle);
    drawClues(puzzle, guessedWords);
    document.getElementById("answers-toggle").onclick = () => {
        if (document.getElementById("answers-toggle").innerHTML == "Show Answers") { 
            showAnswers(puzzle, guessedWords);
        } else {
            hideAnswers(puzzle, guessedWords);
        }
    }
})

const canvas = document.querySelector('#canvas');
const ctx = canvas.getContext('2d');
const boxSize = canvas.width / 20;
ctx.font = "bold 9pt monospace";

function drawPuzzle(puzzle) {
    ctx.fillStyle = '#FFFFFF';
    ctx.strokeStyle = '#000000';
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
    drawNumbers(puzzle);
}

function drawNumbers(puzzle) {
    ctx.textAlign = "left";
    ctx.fillStyle = "#000000";
    for (var i = 0; i < puzzle.length; i ++) { 
        const word = puzzle[i];
        ctx.fillText(i + 1, word.xpos * boxSize, (word.ypos) * boxSize + 10);
    }
}

function drawGuesses(puzzle, guessedWords) {
    ctx.textAlign = "center";
    ctx.fillStyle="#000000";
    // make sure answer button is reset because answers are hidden
    document.getElementById("answers-toggle").innerHTML = "Show Answers"

    for(var i = 0; i < puzzle.length; i ++) {
        const answer = puzzle[i];
        const guess = guessedWords[i];

        if (answer.orientation == Orientation.Horizontal) {
            for (var j = 0; j < guess.length; j ++) {
                ctx.fillText(
                    guess[j], 
                    (answer.xpos + 0.5 + j) * boxSize, 
                    (answer.ypos + 0.6) * boxSize
                );
            }
        } else {
            for (var j = 0; j < guess.length; j ++) {
                ctx.fillText(
                    guess[j], 
                    (answer.xpos + 0.5) * boxSize,
                    (answer.ypos + 0.6 + j) * boxSize
                );
            }
        }
    }
}

// needs to know guessed words to create the listener
function drawClues(puzzle, guessedWords) {
    for (var i = 0; i < puzzle.length; i ++) {
        const word = puzzle[i];
        const clue = document.createElement("p");
        console.log(word.clue);
        clue.innerText = (i + 1) + ". " + word.clue;

        const inputField = document.createElement("input");
        inputField.setAttribute("maxlength", word.word.length);

        ((i) => {
            inputField.addEventListener("keydown", function (e) {
                if (e.code === "Enter") {
                    guessedWords[i] = inputField.value;
                    drawPuzzle(puzzle, guessedWords);
                    drawGuesses(puzzle, guessedWords);
                }
            });
        })(i);

        if (word.orientation == Orientation.Vertical) {
            document.getElementById("down").appendChild(clue)
            document.getElementById("down").appendChild(inputField);
        } else {
            document.getElementById("across").appendChild(clue)
            document.getElementById("across").appendChild(inputField);
        }
    }
}

function showAnswers(puzzle, guessedWords) {
    drawPuzzle(puzzle);
    ctx.fillStyle = "#000000";
    ctx.textAlign = "center";
    document.getElementById("answers-toggle").innerHTML = "Hide Answers"
    for (var j = 0; j < puzzle.length; j ++) {
        const word = puzzle[j];
        const guess = guessedWords[j];
        if (word.orientation == Orientation.Horizontal) {
            for (var i = 0; i < word.word.length; i ++) {
                ctx.fillText(
                    word.word[i], 
                    (word.xpos + 0.5 + i) * boxSize, 
                    (word.ypos + 0.6) * boxSize
                );
            }
        } else {
            for (var i = 0; i < word.word.length; i ++) {
                ctx.fillText(
                    word.word[i], 
                    (word.xpos + 0.5) * boxSize,
                    (word.ypos + 0.6 + i) * boxSize
                );
            }
        }
    }
}

function hideAnswers(puzzle, guessedWords) {
    document.getElementById("answers-toggle").innerHTML = "Show Answers"
    drawPuzzle(puzzle);
    drawGuesses(puzzle, guessedWords);
}

