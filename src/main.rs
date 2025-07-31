//use std::io;
static DT: f64 = 0.00001;
static min: f64 = -100.0;
static max: f64 = 100.0;

fn eulers(y: f64, ddt: f64) -> f64 
{
    return y + DT*ddt;
}
fn is_colliding(obj1: Object, obj2: Object) -> bool
{
    let delta_x: f64 = obj2.position.x - obj1.position.x;
    let delta_y: f64 = obj2.position.y - obj1.position.y;
    let delta_z: f64 = obj2.position.z - obj1.position.z;
    let distance: f64 = f64::powf( f64::powf(delta_x, 2.0) + f64::powf(delta_y,2.0) + f64::powf(delta_z, 2.0), 0.5);

    if distance <= (obj1.radius + obj2.radius)
    {
        return true;
    }
    return false;
}
fn resolve_collision(objects: &mut Vec<Object>)
{
    let len = objects.len();
    
    for i in 1..len+1
    {
        for j in i+1..len+1
        {
            if is_colliding(objects[i-1], objects[j-1])
            {
            
                let net_momentum_x = objects[i-1].mass*objects[i-1].velocity.x + objects[j-1].mass*objects[j-1].velocity.x;
                let net_momentum_y = objects[i-1].mass*objects[i-1].velocity.y + objects[j-1].mass*objects[j-1].velocity.y;


                let e = (objects[i-1].restitution + objects[j-1].restitution)/2.0;
                //objects[i-1].velocity.x = (j.i*e)*(net_momentum_x)/(objects[i-1].mass+objects[j-1].mass) - e*objects[j-1].velocity.x;
                //objects[i-1].velocity.y = (j.i*e)*(net_momentum_y)/(objects[i-1].mass+objects[j-1].mass) - e*objects[j-1].velocity.y;
                
                objects[i-1].velocity.x = (net_momentum_x + objects[j-1].mass*e*(-objects[i-1].velocity.x + objects[j-1].velocity.x))/(objects[i-1].mass + objects[j-1].mass);
                objects[i-1].velocity.y = (net_momentum_y + objects[j-1].mass*e*(-objects[i-1].velocity.y + objects[j-1].velocity.y))/(objects[i-1].mass + objects[j-1].mass);
                
                objects[j-1].velocity.x = net_momentum_x - objects[i-1].mass*objects[i-1].velocity.x;
                objects[j-1].velocity.y = net_momentum_y - objects[i-1].mass*objects[i-1].velocity.y;
                //println!("{}, {}, {}, {}", objects[0].velocity.x, objects[0].velocity.y, objects[1].velocity.x, objects[1].velocity.y)
            }
        }
    }
}
fn update(objects: &mut Vec<Object>)
{
    for i in 1..objects.len()+1
    {
    objects[i-1].velocity.x = eulers(objects[i-1].velocity.x, objects[i-1].acceleration.x);
    objects[i-1].position.x = eulers(objects[i-1].position.x, objects[i-1].velocity.x    );
    
    objects[i-1].velocity.y = eulers(objects[i-1].velocity.y, objects[i-1].acceleration.y);
    objects[i-1].position.y = eulers(objects[i-1].position.y, objects[i-1].velocity.y    );

    objects[i-1].velocity.z = eulers(objects[i-1].velocity.z, objects[i-1].acceleration.z);
    objects[i-1].position.z = eulers(objects[i-1].position.z, objects[i-1].velocity.z    );

    if (objects[i-1].position.x+objects[i-1].radius >= max) | (objects[i-1].position.x-objects[i-1].radius < min)
    {
        objects[i-1].velocity.x *= -1.0;
    }
    if (objects[i-1].position.y+objects[i-1].radius >= max) | (objects[i-1].position.y-objects[i-1].radius < min)
    {
        objects[i-1].velocity.y *= -1.0;
    }
    if (objects[i-1].position.z+objects[i-1].radius >= max) | (objects[i-1].position.z-objects[i-1].radius < min)
    {
        objects[i-1].velocity.z *= -1.0;
    }
    }
}

#[derive(Copy, Clone)]
pub struct Vector
{
    x: f64,
    y: f64,
    z: f64,
}
impl Default for Vector
{
    fn default() -> Vector
    {
        Vector{x: 0.0, y: 0.0, z:0.0}
    }
}


#[derive(Copy, Clone)]
struct Object
{
    mass: f64,
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
    radius: f64,
    restitution: f64,
}

pub trait Update
{
    fn apply_force(&mut self, force: Vector);
    fn reset_force(&mut self);
}
impl Update for Object
{
    fn apply_force(&mut self, force: Vector)
    {
        self.acceleration.x += force.x/self.mass;
        self.acceleration.y += force.y/self.mass;
        self.acceleration.z += force.z/self.mass;
    }
    fn reset_force(&mut self)
    {
        self.acceleration.x = 0.0;
        self.acceleration.y = 0.0;
        self.acceleration.z = 0.0;
    }
}



fn main() 
{
    let mut objects = vec!
    [
        Object{ position: Vector::default(), velocity: Vector::default(), acceleration: Vector::default(), mass : 1.0, radius:5.0, restitution: 1.0},
        Object{ position: Vector::default(), velocity: Vector::default(), acceleration: Vector::default(), mass : 1.0, radius:5.0, restitution: 1.0},
        Object{ position: Vector::default(), velocity: Vector::default(), acceleration: Vector::default(), mass : 1.0, radius:5.0, restitution: 1.0},
    ];
    objects[0].velocity = Vector{ x : -5.0, y: 5.0, z:0.0};
    objects[0].position = Vector{ x:  10.0, y: 0.0, z:0.0};

    objects[1].velocity = Vector{x: 10.0 , y: 5.0, z:0.0};
    objects[1].position = Vector{x: -10.0, y: 0.0, z:0.0};

    objects[2].velocity = Vector{x: -10.0, y: 5.0, z:10.0};
    objects[2].position = Vector{x: -50.0, y: 0.0, z:0.0};

    //let g  = Vector{x: 0.0, y: -9.8};
    let k: f64 = 2.0;
    
    let mut t: f64 = 0.0;
    
    let data_write_period: i64 = 10000;
    let total_time: f64 = 10.0;    
    while t<=total_time 
    {
        let a: f64 = -k*(objects[0].position.y-20.0)/objects[0].mass;
        update(&mut objects);
        resolve_collision(&mut objects);
        
        t += DT;
        //let a = t/DT;
        if ((t/DT) as i64) % data_write_period == 0
        {
            for i in &objects
            {
                println!("{}, {}, {}, {}", i.position.x, i.position.y, i.position.z, i.radius);
            }
            println!("\n");
        }
        
    }        
   
    
}
