import fs from "fs";
import path from "path";

// Read the file `lines` in the root of the project
// and print each line to the console

// Do it this way
fs.readFileSync("lines")
  .toString()
  .split("\n")
  .filter((_, i) => i % 2 === 0)
  .forEach(line => {
    console.log(line);
  });

// Or this way
fs.readFileSync(path.join(__dirname, "../lines"), {
  encoding: "utf-8",
})
  .split("\n")
  .filter((_, i) => i % 2 === 0)
  .forEach(line => {
    console.log(line);
  });

// Or I don't know :)
