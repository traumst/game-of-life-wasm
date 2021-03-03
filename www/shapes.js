///
// STABLE
///
const block = [ // 2x2
    [1, 1],
    [1, 1],
]
const boat = [ // 3x3
    [1, 1, 0],
    [1, 0, 1],
    [0, 1, 0],
]
const beehive = [ // 3x4
    [0, 1, 1, 0],
    [1, 0, 0, 1],
    [0, 1, 1, 0],
]
const loaf = [ // 4x4
    [0, 1, 1, 0],
    [1, 0, 0, 1],
    [0, 1, 0, 1],
    [0, 0, 1, 0],
]

///
// PERIOD 2
///
const blinker = [ // 3x1
    [1],
    [1],
    [1],
]
const toad = [ // 4x4
    [0, 0, 1, 0],
    [1, 0, 0, 1],
    [1, 0, 0, 1],
    [0, 1, 0, 0],
]

///
// PUBLIC
///
function to1d(shape2d) {
    const shape1d = []
    for (let row of shape2d) {
        shape1d.push(...row)
    }
    return shape1d
}
function publicShapeObject(shape2d) {
    const rows = shape2d.length
    const cols = shape2d[0].length
    const shape1d = to1d(shape2d)
    const size = rows * cols
    return {
        shape1d,
        shape2d,
        rows,
        cols,
        size,
    }
}
function publicShapes() {
    return {
        // STABLE
        boat: publicShapeObject(boat),
        loaf: publicShapeObject(loaf),
        block: publicShapeObject(block),
        beehive: publicShapeObject(beehive),
        // PERIOD 2
        toad: publicShapeObject(toad),
        blinker: publicShapeObject(blinker),
    }
}
module.exports = publicShapes()