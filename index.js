import init, {create_puzzle, Orientation} from "./pkg/crossyword.js";
init().then(() => {
    const puzzle = create_puzzle();
    drawPuzzle(puzzle);
})

function drawPuzzle(puzzle) {
    const canvas = document.querySelector('#canvas');
    const ctx = canvas.getContext('2d');

    const boxSize = canvas.width / 20;

    ctx.fillStyle = '#FFFFFF';
    ctx.strokeStyle = '#000000';
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
            // add clues
            document.getElementById("horizontal-clues").innerHTML
                += word.clue + "\n";
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
            // add clues
            document.getElementById("vertical-clues").innerHTML
                += word.clue + "\n";
        }
        
    }
}

