import init, {create_puzzle} from "./pkg/crossyword.js";
console.log("test");
init().then(() => {
    let puzzle = create_puzzle();
    console.log(puzzle);
})
