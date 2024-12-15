use glam::{vec3, Mat4, Vec3};

struct Grid {
    voxels: Vec<u8>,
    width: usize,
    height: usize,
    depth: usize,
}

struct Ray {
    position: Vec3,
    direction: Vec3,
}

#[derive(Debug)]
struct Hit {
    position: Vec3,
    normal: Vec3,
}

impl Ray {
    pub fn calculate_hit(&self, grid: &Grid) -> Option<Hit> {
        let mut position = self.position;
        let difference = self.position - (self.position + self.direction);
        let mut current_voxel = self.position.floor().as_ivec3();

        let step = difference.signum().as_ivec3();
        let next_voxel_boundary = current_voxel.as_vec3() + 0.5 * step.as_vec3();
        let mut tmax = (next_voxel_boundary - self.position) / difference;
        let tdelta = step.as_vec3() / difference;

        let mut normal;

        for _ in 0..100 {
            if tmax.x < tmax.y {
                if tmax.x < tmax.z {
                    current_voxel.x += step.x;
                    tmax.x += tdelta.x;
                    position.x += step.x as f32;

                    normal = Vec3::new(-step.x as f32, 0.0, 0.0);
                } else {
                    current_voxel.z += step.z;
                    tmax.z += tdelta.z;
                    position.z += step.z as f32;

                    normal = Vec3::new(0.0, 0.0, -step.z as f32);
                }
            } else if tmax.y < tmax.z {
                current_voxel.y += step.y;
                tmax.y += tdelta.y;
                position.y += step.y as f32;

                normal = Vec3::new(0.0, -step.y as f32, 0.0);
            } else {
                current_voxel.z += step.z;
                tmax.z += tdelta.z;
                position.z += step.z as f32;

                normal = Vec3::new(0.0, 0.0, -step.z as f32);
            }

            let x_coord = current_voxel.x.clamp(0, grid.width as i32 - 1) as usize;
            let y_coord = current_voxel.y.clamp(0, grid.height as i32 - 1) as usize;
            let z_coord = current_voxel.z.clamp(0, grid.depth as i32 - 1) as usize;

            position -= normal;

            if grid.voxels[x_coord + y_coord * grid.width + z_coord * grid.width * grid.height] == 1
            {
                return Some(Hit { position, normal });
            }
        }

        return None;
    }
}

impl Grid {
    pub fn square(width: usize, height: usize, depth: usize) -> Grid {
        let mut voxels = vec![0; width * height * depth];
        let h = height / 4;
        let w = width / 4;
        let d = depth / 4;
        for y in h * 1..h * 3 {
            for x in w * 1..w * 3 {
                for z in d * 1..d * 3 {
                    voxels[x + y * width + z * (width * height)] = 1;
                }
            }
        }

        Grid {
            voxels,
            width,
            depth,
            height,
        }
    }

    pub fn circle(width: usize, height: usize, depth: usize) -> Grid {
        let mut voxels = vec![0; width * height * depth];

        let center = vec3(width as f32, height as f32, depth as f32) * 0.5;

        for y in 0..height {
            for x in 0..width {
                for z in 0..depth {
                    let point = vec3(x as f32, y as f32, z as f32);
                    if point.distance_squared(center) < 10.0f32.powf(2.0) {
                        voxels[x + y * width + z * (width * height)] = 1;
                    }
                }
            }
        }

        Grid {
            voxels,
            width,
            depth,
            height,
        }
    }
}

fn main() {
    let grid = Grid::circle(40, 40, 40);
}

/*ray.direction - 2.0 * hit.normal.dot(ray.direction) * hit.normal*/
