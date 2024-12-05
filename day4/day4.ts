import { readFileSync } from 'fs';

function findAdjacentIndexes(x: number, y: number, max: number): [number, number][][] {
    let indices: [number, number][][] = [];

    if (x + 3 < max) {
        indices.push([[x, y], [x + 1, y], [x + 2, y], [x + 3, y]]);
    }

    if (x - 3 >= 0) {
        indices.push([[x - 3, y], [x - 2, y], [x - 1, y], [x, y]]);
    }

    if (y + 3 < max) {
        indices.push([[x, y], [x, y + 1], [x, y + 2], [x, y + 3]]);
    }

    if (y - 3 >= 0) {
        indices.push([[x, y - 3], [x, y - 2], [x, y - 1], [x, y]]);
    }

    if (x + 3 < max && y + 3 < max) {
        indices.push([[x, y], [x + 1, y + 1], [x + 2, y + 2], [x + 3, y + 3]]);
    }

    if (x - 3 >= 0 && y - 3 >= 0) {
        indices.push([[x, y], [x - 1, y - 1], [x - 2, y - 2], [x - 3, y - 3]]);
    }

    if (x + 3 < max && y - 3 >= 0) {
        indices.push([[x, y], [x + 1, y - 1], [x + 2, y - 2], [x + 3, y - 3]]);
    }

    if (x - 3 >= 0 && y + 3 < max) {
        indices.push([[x, y], [x - 1, y + 1], [x - 2, y + 2], [x - 3, y + 3]]);
    }

    return indices;
}

function part1(grid: string[]) {
    let count = 0;

    let max = grid.length;

    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            let adjacentWords = findAdjacentIndexes(i, j, max);
        
            let words = adjacentWords.map(indices => indices.map(([x, y]) => grid[x][y]).join(""));
            count += words.filter(word => word === "XMAS" || word === "SAMX").length / 2;
        }
    }

    console.log("Part 1 result is", count);
}

function part2(grid: string[]) {
    let count = 0;

    let max = grid.length;

    for (let i = 1; i < grid.length; i++) {
        for (let j = 1; j < grid[i].length; j++) {
            if (grid[i][j] === "A" && i < max - 1 && j < max - 1) {
                if (grid[i - 1][j + 1] === "M" && grid[i + 1][j - 1] === "S") {
                    if (grid[i - 1][j - 1] === "M" && grid[i + 1][j + 1] === "S") {
                        count++;
                    }
                    else if (grid[i - 1][j - 1] === "S" && grid[i + 1][j + 1] === "M") {
                        count++;
                    }
                }
                else if (grid[i - 1][j + 1] === "S" && grid[i + 1][j - 1] === "M") {
                    if (grid[i - 1][j - 1] === "M" && grid[i + 1][j + 1] === "S") {
                        count++;
                    }
                    else if (grid[i - 1][j - 1] === "S" && grid[i + 1][j + 1] === "M") {
                        count++;
                    }
                }
            }
        }
    }

    console.log("Part 2 result is", count);
}

let path = process.argv[2];
let input = readFileSync(path, 'utf-8').trim().split("\n");

part1(input);
part2(input);
