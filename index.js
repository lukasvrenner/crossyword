import init, {create_puzzle} from "../pkg/crossyword.js";
init().then(() => {
    console.log("test");
    let puzzle = create_puzzle();
    console.log(puzzle);
})
