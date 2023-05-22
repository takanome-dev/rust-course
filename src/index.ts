// // ---------------------------------- ITERATE THROUGH FILE ----------------------------------

// import fs from "fs";
// import path from "path";

// // Read the file `lines` in the root of the project
// // and print each line to the console

// // Do it this way
// fs.readFileSync("lines")
//   .toString()
//   .split("\n")
//   .filter((_, i) => i % 2 === 0)
//   .forEach(line => {
//     console.log(line);
//   });

// // Or this way
// fs.readFileSync(path.join(__dirname, "../lines"), {
//   encoding: "utf-8",
// })
//   .split("\n")
//   .filter((_, i) => i % 2 === 0)
//   .forEach(line => {
//     console.log(line);
//   });

// // iterate through the file `lines` and print every other lines
// // but skip the first two lines
// fs.readFileSync(path.join(__dirname, "../lines"), {
//   encoding: "utf-8",
// })
//   .split("\n")
//   .filter((_, i) => i % 2 === 0)
//   .filter((_, i) => i > 1 && i < 4)
//   .forEach(line => {
//     console.log(line);
//   });

// // ------------------------------------- ENUMS ---------------------------------------

// enum Color {
//   Red,
//   Green,
//   Blue,
//   Yellow,
// }

// function printColor(color: Color) {
//   switch (color) {
//     case Color.Red:
//       console.log("red");
//       break;
//     case Color.Green:
//       console.log("green");
//       break;
//     case Color.Blue:
//       console.log("blue");
//       break;
//   }
// }

// printColor(Color.Yellow);

type Custom = {
  age: number;
  name: string;
};

type Item = number | string | Custom;

const append = (items: Item[]) => {
  return items.push("Hello Fem!");
};

const items: Item[] = [];
console.log({ items });
append(items);
console.log({ items });

// const nums = [1, 2, 3, 4, 5];
const numbers: number[] = [];
console.log({ items });
append(numbers);
console.log({ items });
