pub struct Shape {
    pub name: String,
    pub matrix: Vec<Vec<u32>>
}

impl Shape {
    pub fn new(name: String, matrix: Vec<Vec<u32>>) -> Shape {
        Shape {
            name,
            matrix
        }
    }
}

pub struct Shapes {
    pub block: Shape,
    pub beehive: Shape,
    pub loaf: Shape,
    pub boat: Shape,
    pub tub: Shape,
    pub blinker: Shape,
    pub toad: Shape,
    pub beacon: Shape,
    pub glider: Shape,
    pub lwss: Shape,
    pub mwss: Shape,
    pub hwss: Shape,
}

impl Shapes {
    pub fn new() -> Shapes {
        Shapes {
            block: Shape {
                name: "block".to_string(),
                matrix: vec![
                    vec![0, 0, 0, 0],
                    vec![0, 1, 1, 0],
                    vec![0, 1, 1, 0],
                    vec![0, 0, 0, 0]
                ]
            },

            beehive: Shape {
                name: "beehive".to_string(),
                matrix: vec![
                    vec![0, 0, 0, 0, 0, 0],
                    vec![0, 0, 1, 1, 0, 0],
                    vec![0, 1, 0, 0, 1, 0],
                    vec![0, 0, 1, 1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0]
                ]
            },

            loaf: Shape {
                name: "loaf".to_string(),
                matrix: vec![
                    vec![0, 0, 0, 0, 0, 0],
                    vec![0, 0, 1, 1, 0, 0],
                    vec![0, 1, 0, 0, 1, 0],
                    vec![0, 0, 1, 0, 1, 0],
                    vec![0, 0, 0, 1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0]
                ]
            },

            boat: Shape {
                name: "boat".to_string(),
                matrix: vec![
                    vec![0, 0, 0, 0, 0],
                    vec![0, 1, 1, 0, 0],
                    vec![0, 1, 0, 1, 0],
                    vec![0, 0, 1, 0, 0],
                    vec![0, 0, 0, 0, 0]
                ]
            },

            tub: Shape {
                name: "tub".to_string(),
                matrix: vec![
                    vec![0, 0, 0, 0, 0],
                    vec![0, 0, 1, 0, 0],
                    vec![0, 1, 0, 1, 0],
                    vec![0, 0, 1, 0, 0],
                    vec![0, 0, 0, 0, 0]
                ]
            },

            blinker: Shape {
                name: "blinker".to_string(),
                matrix: vec![
                    vec![0,0,0,0,0],
                    vec![0,1,1,1,0],
                    vec![0,0,0,0,0]
                ]
            },

            toad: Shape {
                name: "toad".to_string(),
                matrix: vec![
                    vec![0, 0, 0, 0, 0, 0],
                    vec![0, 0, 1, 1, 1, 0],
                    vec![0, 1, 1, 1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0]
                ]
            },

            beacon: Shape {
                name: "beacon".to_string(),
                matrix: vec![
                    vec![1, 1, 0, 0],
                    vec![1, 1, 0, 0],
                    vec![0, 0, 1, 1],
                    vec![0, 0, 1, 1]
                ]
            },

            glider: Shape {
                name: "glider".to_string(),
                matrix: vec![
                    vec![1, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 1, 1, 0, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0]
                ]
            },

            lwss: Shape {
                name: "lwss".to_string(),
                matrix: vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                ]
            },

            mwss: Shape {
                name: "mwss".to_string(),
                matrix: vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                ]
            },

            hwss: Shape {
                name: "hwss".to_string(),
                matrix: vec![
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                ]
            }
        }
    }
}