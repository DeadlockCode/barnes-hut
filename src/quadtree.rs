use std::ops::Range;

use crate::{body::Body, partition::Partition};
use ultraviolet::Vec2;

#[derive(Clone, Copy)]
pub struct Quad {
    pub center: Vec2,
    pub size: f32,
}

impl Quad {
    pub fn new_containing(bodies: &[Body]) -> Self {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for body in bodies {
            min_x = min_x.min(body.pos.x);
            min_y = min_y.min(body.pos.y);
            max_x = max_x.max(body.pos.x);
            max_y = max_y.max(body.pos.y);
        }

        let center = Vec2::new(min_x + max_x, min_y + max_y) * 0.5;
        let size = (max_x - min_x).max(max_y - min_y);

        Self { center, size }
    }

    pub fn into_quadrant(mut self, quadrant: usize) -> Self {
        self.size *= 0.5;
        self.center.x += ((quadrant & 1) as f32 - 0.5) * self.size;
        self.center.y += ((quadrant >> 1) as f32 - 0.5) * self.size;
        self
    }

    pub fn subdivide(&self) -> [Quad; 4] {
        [0, 1, 2, 3].map(|i| self.into_quadrant(i))
    }
}

#[derive(Clone)]
pub struct Node {
    pub children: usize,
    pub next: usize,
    pub pos: Vec2,
    pub mass: f32,
    pub quad: Quad,
    pub bodies: Range<usize>,
}

impl Node {
    pub fn new(next: usize, quad: Quad, bodies: Range<usize>) -> Self {
        Self {
            children: 0,
            next,
            pos: Vec2::zero(),
            mass: 0.0,
            quad,
            bodies,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children == 0
    }

    pub fn is_branch(&self) -> bool {
        self.children != 0
    }

    pub fn is_empty(&self) -> bool {
        self.mass == 0.0
    }
}

pub struct Quadtree {
    pub t_sq: f32,
    pub e_sq: f32,
    pub leaf_capacity: usize,
    pub nodes: Vec<Node>,
    pub parents: Vec<usize>,
}

impl Quadtree {
    pub const ROOT: usize = 0;

    pub fn new(theta: f32, epsilon: f32, leaf_capacity: usize) -> Self {
        Self {
            t_sq: theta * theta,
            e_sq: epsilon * epsilon,
            leaf_capacity,
            nodes: Vec::new(),
            parents: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.parents.clear();
    }

    pub fn subdivide(&mut self, node: usize, bodies: &mut [Body], range: Range<usize>) {
        let center = self.nodes[node].quad.center;

        let mut split = [range.start, 0, 0, 0, range.end];

        let predicate = |body: &Body| body.pos.y < center.y;
        split[2] = split[0] + bodies[split[0]..split[4]].partition(predicate);

        let predicate = |body: &Body| body.pos.x < center.x;
        split[1] = split[0] + bodies[split[0]..split[2]].partition(predicate);
        split[3] = split[2] + bodies[split[2]..split[4]].partition(predicate);

        self.parents.push(node);
        let children = self.nodes.len();
        self.nodes[node].children = children;

        let nexts = [
            children + 1,
            children + 2,
            children + 3,
            self.nodes[node].next,
        ];
        let quads = self.nodes[node].quad.subdivide();
        for i in 0..4 {
            let bodies = split[i]..split[i + 1];
            self.nodes.push(Node::new(nexts[i], quads[i], bodies));
        }
    }

    pub fn propagate(&mut self) {
        for &node in self.parents.iter().rev() {
            let i = self.nodes[node].children;

            self.nodes[node].pos = self.nodes[i].pos
                + self.nodes[i + 1].pos
                + self.nodes[i + 2].pos
                + self.nodes[i + 3].pos;

            self.nodes[node].mass = self.nodes[i].mass
                + self.nodes[i + 1].mass
                + self.nodes[i + 2].mass
                + self.nodes[i + 3].mass;
        }
        for node in &mut self.nodes {
            node.pos /= node.mass.max(f32::MIN_POSITIVE);
        }
    }

    pub fn build(&mut self, bodies: &mut [Body]) {
        self.clear();

        let quad = Quad::new_containing(bodies);
        self.nodes.push(Node::new(0, quad, 0..bodies.len()));

        let mut node = 0;
        while node < self.nodes.len() {
            let range = self.nodes[node].bodies.clone();
            if range.len() > self.leaf_capacity {
                self.subdivide(node, bodies, range);
            } else {
                for i in range {
                    self.nodes[node].pos += bodies[i].pos * bodies[i].mass;
                    self.nodes[node].mass += bodies[i].mass;
                }
            }
            node += 1;
        }

        self.propagate();
    }

    pub fn acc(&self, pos: Vec2, bodies: &[Body]) -> Vec2 {
        let mut acc = Vec2::zero();

        let mut node = Self::ROOT;
        loop {
            let n = self.nodes[node].clone();

            let d = n.pos - pos;
            let d_sq = d.mag_sq();

            if n.quad.size * n.quad.size < d_sq * self.t_sq {
                let denom = (d_sq + self.e_sq) * d_sq.sqrt();
                acc += d * (n.mass / denom);

                if n.next == 0 {
                    break;
                }
                node = n.next;
            } else if n.is_leaf() {
                for i in n.bodies {
                    let body = &bodies[i];
                    let d = body.pos - pos;
                    let d_sq = d.mag_sq();

                    let denom = (d_sq + self.e_sq) * d_sq.sqrt();
                    acc += d * (body.mass / denom).min(f32::MAX);
                }

                if n.next == 0 {
                    break;
                }
                node = n.next;
            } else {
                node = n.children;
            }
        }

        acc
    }
}
