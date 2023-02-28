use ggez::graphics::{Color, Mesh, MeshBuilder};
use ggez::mint::Point2;
use ggez::Context;
use std::f32::consts::PI;
use std::time::Instant;

const START_LENGTH: f32 = 100.0;
const LENGTH_LOST: f32 = 0.8;

const START_WIDTH: f32 = 20.0;
const WIDTH_LOST: f32 = 0.8;

const ANGLE_DIF: f32 = PI / 8.0;

pub struct Tree {
    main_branch: Branch,

    size: i32,
}

impl Tree {
    pub fn new() -> Tree {
        let mut tree = Tree {
            main_branch: Branch::new(),
            size: 0,
        };

        for _ in 0..3 {
            tree.add_sub_branch();
        }
        tree
    }

    pub fn generate_mesh(&self, ctx: &mut Context) -> Mesh {
        let mut builder = MeshBuilder::new();

        let instant = Instant::now();
        self.main_branch.generate_mesh(&mut builder, 0, self.size);
        println!("generate {}", instant.elapsed().as_secs_f64());

        let instant = Instant::now();
        let mesh = Mesh::from_data(ctx, builder.build());
        println!("from data {}", instant.elapsed().as_secs_f64());

        mesh
    }

    pub fn add_sub_branch(&mut self) {
        self.size += 1;
        self.main_branch.add_sub_branch();
    }
}

pub struct Branch {
    start_point: Point2<f32>,
    length: f32,
    angle: f32,
    width: f32,
    sub_branches: Vec<Branch>,
}

impl Branch {
    pub fn new() -> Branch {
        Branch {
            start_point: Point2 { x: 0.0, y: 0.0 },
            length: START_LENGTH,
            angle: PI / 2.0,
            width: START_WIDTH,
            sub_branches: vec![],
        }
    }

    pub fn from_parent(paren_branch: &Branch, angle_dif: f32) -> Branch {
        let angle = paren_branch.angle + angle_dif;
        let start_point = Point2 {
            x: paren_branch.angle.cos() * paren_branch.length + paren_branch.start_point.x,
            y: paren_branch.angle.sin() * paren_branch.length + paren_branch.start_point.y,
        };

        Branch {
            start_point,
            length: paren_branch.length * LENGTH_LOST,
            angle,
            width: paren_branch.width * WIDTH_LOST,
            sub_branches: vec![],
        }
    }

    pub fn add_sub_branch(&mut self) {
        if self.sub_branches.is_empty() {
            let sub_branch = Branch::from_parent(self, ANGLE_DIF);
            self.sub_branches.push(sub_branch);

            let sub_branch = Branch::from_parent(self, -ANGLE_DIF);
            self.sub_branches.push(sub_branch);
        } else {
            for sub_branch in &mut self.sub_branches {
                sub_branch.add_sub_branch();
            }
        }
    }

    pub fn generate_mesh(&self, builder: &mut MeshBuilder, branch_depth: i32, branch_count: i32) {
        builder
            .line(
                &vec![
                    self.start_point,
                    Point2 {
                        x: self.angle.cos() * self.length + self.start_point.x,
                        y: self.angle.sin() * self.length + self.start_point.y,
                    },
                ],
                self.width,
                Color::new(
                    branch_depth as f32 / branch_count as f32,
                    0.,
                    branch_depth as f32 / branch_count as f32,
                    1.,
                ),
            )
            .expect("could not create Line");

        for sub_branch in &self.sub_branches {
            sub_branch.generate_mesh(builder, branch_depth + 1, branch_count);
        }
    }
}
