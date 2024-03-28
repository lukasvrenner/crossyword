import init, {create_puzzle} from "../pkg/crossyword.js";
init().then(() => {
    let puzzle = create_puzzle();
    console.log(puzzle);
})
