use glam::{vec3, Mat3, Vec3};
use ratatui::crossterm::style::Color;
use ratatui::text::{Line, Span, Text};
use std::time::Duration;
use std::thread;

use ratatui::{
    style::Stylize,
    widgets::Paragraph,
};

#[derive(Debug,Copy, Clone)]
pub struct Transform{
    pub rotation: Vec3,
    pub position: Vec3,
    pub scale: Vec3,
}

#[derive(Debug,Copy, Clone)]
struct Light{
    position: Vec3,
    energy: f32,
    radius:f32
}
#[derive(Clone,Debug)]
pub struct World{
    pub camera: Camera,
    meshes: Vec<Mesh>,
    pub mat_atals: Vec<Material>,
    Light: Option<Vec<Light>>,
    triangels_world: Option<Vec<Triangel>>,
}



#[derive(Copy, Clone,Debug)]
pub struct Camera{
    width: i32,
    hight: i32,
    pub origin: Vec3,
    pub rotation: Vec3,
    spread: f32,
    focal_lenth: f32
}

#[derive(Debug,Copy, Clone)]
struct Ray{
    origin: Vec3,
    direction: Vec3
}
#[derive(Copy, Clone,Debug)]
struct Triangel{
    a: glam::Vec3,
    b:glam::Vec3,
    c:glam::Vec3
}

type Material= Color;

#[derive( Clone,Debug)]
struct Mesh{
    triangels_local: Vec<Triangel>,
    triangels_world: Vec<Triangel>,
    rotation: Vec3,
    position: Vec3,
    scale: Vec3,
    material_id: Option<i32>,

}

pub fn rotate_degrees(vectro:Vec3,rotation:Vec3)->Vec3{
    let rot_raidnts = rotation*0.01745329;
    rotate(vectro,rot_raidnts)
}
fn rotate(vectro:Vec3,rotation:Vec3)-> Vec3{
    let mut orentaion = vectro;
    let x_rotion = Mat3::from_rotation_x(rotation.x);
    let y_rotion = Mat3::from_rotation_y(rotation.y);
    let z_rotion = Mat3::from_rotation_z(rotation.z);
    orentaion = x_rotion*orentaion;
    orentaion = y_rotion*orentaion;
    orentaion = z_rotion*orentaion;
    orentaion
}


impl World {
    pub fn empty_with_defult_materials(cam:Camera,capacety:usize)  -> World{
        let meshes:Vec<Mesh> = Vec::with_capacity(capacety);
        //let materilas: Vec<Material> = vec!["#".to_string(),"u".black().to_string(),"e".red().to_string(),"h".green().to_string()];
        let materilas: Vec<Color> = vec![Color::Black,Color::Green,Color::White,Color::Red];
        World{ camera:cam,meshes,mat_atals:materilas, Light:None,triangels_world:None}
    } 
    pub fn world_mesh(mut self) -> (Option<Vec<Triangel>>,World){
        let capacety = self.meshes.len();
        let mut world_mesh:Vec<Triangel>= Vec::with_capacity(capacety);
        for m in self.meshes.clone(){
            let m=m.update();
            let mut trigs= m.triangels_world;
            world_mesh.append(&mut trigs);
        }
        self.triangels_world = Some(world_mesh);
        return  (self.triangels_world.clone(),self);
    }
}



impl Mesh{
    fn update(self)->Mesh{
        let tirig_size = self.triangels_local.len();
        let mut  triangels_world = Vec::with_capacity(tirig_size);
        let p = self.position;
        let r = self.rotation;
        let s = self.scale;
        for t in self.triangels_local.clone(){
            let array = [t.a,t.b,t.c];
            let mut new_array = [t.a,t.b,t.c];
            let mut i = 0;
            for v in array{
                let vn = rotate_degrees(v,r );
                let vn = vn*s;
                let vn = vn +p;
                new_array[i] = vn;
                i +=1;
            }
            triangels_world.push(Triangel{a:new_array[0],b:new_array[1],c:new_array[2]})
        }
        Mesh{triangels_local:self.triangels_local,triangels_world,rotation:r,position:p,scale:s,material_id:None}
    }
}


impl Mesh{
    fn new_box(position:Vec3,scale: Vec3, rotation:Vec3) -> Mesh{
        let triangels_local = vec![
            Triangel{a:Vec3{x:1.0,y:-1.0,z:1.0},b:Vec3{x:-1.0,y:-1.0,z:1.0},c:Vec3{x:1.0,y:-1.0,z:-1.0}},
            Triangel{a:Vec3{x:-1.0,y:-1.0,z:-1.0},b:Vec3{x:-1.0,y:-1.0,z:1.0},c:Vec3{x:1.0,y:-1.0,z:-1.0}},//bottom
            Triangel{a:Vec3{x:1.0,y:1.0,z:1.0},b:Vec3{x:-1.0,y:1.0,z:1.0},c:Vec3{x:1.0,y:1.0,z:-1.0}},
            Triangel{a:Vec3{x:-1.0,y:1.0,z:-1.0},b:Vec3{x:-1.0,y:1.0,z:1.0},c:Vec3{x:1.0,y:1.0,z:-1.0}},//top
            Triangel{a:Vec3{x:1.0,y:-1.0,z:-1.0},b:Vec3{x:1.0,y:-1.0,z:1.0},c:Vec3{x:1.0,y:1.0,z:-1.0}},
            Triangel{a:Vec3{x:1.0,y:1.0,z:1.0},b:Vec3{x:1.0,y:1.0,z:-1.0},c:Vec3{x:1.0,y:-1.0,z:1.0}},//side  x=1
            Triangel{a:Vec3{x:-1.0,y:-1.0,z:1.0},b:Vec3{x:-1.0,y:-1.0,z:-1.0},c:Vec3{x:-1.0,y:1.0,z:-1.0}},
            Triangel{a:Vec3{x:-1.0,y:1.0,z:1.0},b:Vec3{x:-1.0,y:1.0,z:-1.0},c:Vec3{x:-1.0,y:-1.0,z:1.0}},//side  x=-1
            Triangel{a:Vec3{x:1.0,y:-1.0,z:1.0},b:Vec3{x:-1.0,y:-1.0,z:1.0},c:Vec3{x:1.0,y:1.0,z:1.0}},
            Triangel{a:Vec3{x:1.0,y:1.0,z:1.0},b:Vec3{x:-1.0,y:1.0,z:1.0},c:Vec3{x:-1.0,y:-1.0,z:1.0}},//side  z=1
            Triangel{a:Vec3{x:1.0,y:-1.0,z:-1.0},b:Vec3{x:-1.0,y:-1.0,z:-1.0},c:Vec3{x:-1.0,y:1.0,z:-1.0}},
            Triangel{a:Vec3{x:1.0,y:1.0,z:-1.0},b:Vec3{x:-1.0,y:1.0,z:-1.0},c:Vec3{x:1.0,y:-1.0,z:-1.0}},//side  z=-1

        ];

        let tirig_size = triangels_local.len();
        let triangels_world = Vec::with_capacity(tirig_size);
        let bad_mesh = Mesh{triangels_local, triangels_world,rotation,position,scale,material_id:None};
        
        bad_mesh.update()
    }
}



impl Ray {

    fn new_ray_looking_at(origin:Vec3, point:Vec3)->Ray{
        let direction = point-origin;
        Ray{origin,direction}
    }
    fn render_world(self,world:World)->i32{
         todo!("everything");

    }
}

impl Camera {
    pub fn get_transform(&self) -> Transform{
        Transform{rotation:self.rotation,position:self.origin,scale:Vec3::ZERO}
    } 
    pub fn apply_transform(self,t:Transform) -> Camera{
        Camera{spread:self.spread,width:self.width,hight:self.hight,origin:t.position,rotation:t.rotation,focal_lenth:self.focal_lenth}
    } 
    fn rotate_degrees(self,rotation:Vec3)->Camera{
        let rot_raidnts = rotation*0.01745329;
        self.rotate(rot_raidnts)
    }
    fn rotate(self,rotation:Vec3)-> Camera{
        let mut orentaion = self.rotation;
        let x_rotion = Mat3::from_rotation_x(rotation.x);
        let y_rotion = Mat3::from_rotation_y(rotation.y);
        let z_rotion = Mat3::from_rotation_z(rotation.z);
        orentaion = x_rotion*orentaion;
        orentaion = y_rotion*orentaion;
        orentaion = z_rotion*orentaion;
        Camera{width:self.width,hight:self.hight,origin:self.origin,rotation:orentaion,spread:self.spread,focal_lenth:self.focal_lenth}
    }
    pub fn local_to_world(&self,point:Vec3)->Vec3{
        let local_origen = self.origin;
        point + local_origen
    }
    
}

fn trangel_intersetion(ray: Ray, triangel: Triangel)-> 
    Option<Vec3>{
        let e1 = triangel.b -triangel.a;
        let e2 = triangel.c -triangel.a;
        let ray_cross_e2 = ray.direction.cross(e2);
        let det = e1.dot(ray_cross_e2);
        if det> -f32::EPSILON && det<f32::EPSILON{
            return None;
        }
        let inv_detect = 1.0/det;
        let s= ray.origin - triangel.a;
        let u = inv_detect * s.dot(ray_cross_e2);
        if !(0.0..=1.0).contains(&u){
            return None;
        }
        let s_cross_e1 = s.cross(e1);
        let v = inv_detect *ray.direction.dot(s_cross_e1);
        if v < 0.0 || u+v >1.0{
            return None;
        }
        let t = inv_detect * e2.dot(s_cross_e1);
        if t > f32::EPSILON{
            let intersetion_point = ray.origin + ray.direction *t;
            Some(intersetion_point)
        }else {
            None
        }

}

fn generate_ray_array_offset(camera: Camera)-> Vec<Ray>{
    // initilze retutrn array
    let mut return_array: Vec<Ray> = Vec::with_capacity((camera.width*camera.hight).try_into().unwrap());
    // vectros acrros the entire viewport
    let render_width = vec3(camera.spread,0.0,0.0); 
    let render_hight = vec3(0.0,-camera.spread,0.0);
    // find deltes
    let x_delta = render_width/ camera.width as f32;
    let y_delta = render_hight/ camera.hight as f32;
    //find center and top left corner
    //let center_of_cam = camera.local_to_world(vec3(0.0, 0.0, 0.0));
    let center_of_cam = camera.origin;
    let top_left_corner = center_of_cam - vec3(0.0, 0.0, camera.focal_lenth) - render_hight/2.0 -render_width/2.0;
    for y in 0..camera.hight {
        for x in 0..camera.width {
            let displacment = x_delta*x as f32 + y_delta*y as f32;
            let point = camera.local_to_world(rotate_degrees(top_left_corner +displacment, camera.rotation)); //  maby fix later 
            let new_ray = Ray::new_ray_looking_at(camera.origin, point);
            return_array.push(new_ray);
            //println!("{:09} {:09} {:09}",point.x,point.y,point.z);
        }
    }
    return_array

}

fn generate_ray_array_offset_light(camera: Camera)-> Vec<Ray>{
    // initilze retutrn array
    let mut return_array: Vec<Ray> = Vec::with_capacity((camera.width*camera.hight).try_into().unwrap());
    // vectros acrros the entire viewport
    let render_width = vec3(camera.spread,0.0,0.0); 
    let render_hight = vec3(0.0,-camera.spread,0.0);
    // find deltes
    let x_delta = render_width/ camera.width as f32;
    let y_delta = render_hight/ camera.hight as f32;
    //find center and top left corner
    let center_of_cam = camera.origin;
    let top_left_corner = center_of_cam - vec3(0.0, 0.0, camera.focal_lenth) - render_hight/2.0 -render_width/2.0;
    for y in 0..camera.hight {
        for x in 0..camera.width {
            let displacment = x_delta*x as f32 + y_delta*y as f32;
            let point = camera.local_to_world(rotate_degrees(top_left_corner +displacment, camera.rotation)); //  maby fix later 
            let new_ray = Ray::new_ray_looking_at(camera.origin, point);
            return_array.push(new_ray);
            //println!("{:09} {:09} {:09}",point.x,point.y,point.z);
        }
    }
    return_array

}
pub fn render_world_mesh(world:World){
    let camera = world.camera;
    let size = (camera.width*camera.hight) as usize;
    let all_size = size*world.meshes.len();
    let ray_array = generate_ray_array_offset(camera);
    let mut world_array :Vec<Vec<i32>> = Vec::with_capacity(all_size);
    let r = match world.triangels_world {
        Some(t)=>t,
        None=>world.world_mesh().0.unwrap().clone(),
    };
    //println!("{:?}",world);

}


fn render_mesh(mesh:Mesh, camera: Camera) ->Vec<i32>{
    let size = (camera.width*camera.hight) as usize;
    let mut display_array = Vec::with_capacity(size);
    let ray_array = generate_ray_array_offset(camera);
    for i in 0..size {
        display_array.push(0);
    }
    for t in mesh.triangels_world{
        display_array = render_triange_vect_material(t, camera, display_array,ray_array.clone(),mesh.material_id);

    }
    display_array

}
fn render_triange_vect(triangel: Triangel, mut vect:Vec<i32>, ray_array: Vec<Ray>)-> Vec<i32>{
    let mut index = 0;
    //println!("ray_array {:#?}", ray_array);
    for i in ray_array{
        let result = trangel_intersetion(i, triangel);
        if result.is_some(){
            vect[index] +=1;
        }
        index += 1;
    }
    vect
    
}
fn render_triange_vect_material(triangel: Triangel,camera:Camera, mut vect:Vec<i32>, ray_array: Vec<Ray>, material: Option<i32>)-> Vec<i32>{
    let mut index = 0;
    let mut m_int = 1;
    if material.is_some(){
        m_int = material.unwrap();
    }
    //println!("material  {:#?}", m_int);
    for i in ray_array{
        let result = trangel_intersetion(i, triangel);
        if result.is_some(){
            vect[index] = m_int;
        }
        index += 1;
    }
    vect
    
}


fn render_triange(triangel: Triangel,camera:Camera)-> String{
    let mut str = "".to_string();
    let mut index = 0;
    let ray_array = generate_ray_array_offset(camera);
    //println!("ray_array {:#?}", ray_array);
    for i in ray_array{
        let result = trangel_intersetion(i, triangel);
        if result.is_some(){
            str += "##";
        }else {
            str += "..";
        }
        index += 1;
        if index== camera.width{
            index =0;
            str += "\n";
        }
    }
    str
    
}

pub fn test_render_in_text_mode() {
    //let tri = Triangel{a:Vec3{x:1.0,y:0.0,z:0.0},b:Vec3{x:0.0,y:1.0,z:0.0},c:Vec3{x:-1.0,y:0.0,z:0.0}};
    let tri = Triangel{a:Vec3{x:0.0,y:1.0,z:-1.0},b:Vec3{x:1.0,y:0.0,z:-1.0},c:Vec3{x:-1.0,y:0.0,z:-1.0}};
    let mut camera = Camera{origin:Vec3 { x: 0.0, y: 0.0, z: 0.0 },rotation:Vec3 { x: 0.0, y: 0.0, z: 0.0 },width:50,hight:50,spread:40.0,focal_lenth:5.0};
    render_triange(tri, camera);
    for i in 0..1{
        //tri = triangel_sligtly_discombulated(tri);
        camera =camera.rotate_degrees(vec3(0.0, 0.0, 0.0));
        println!("{}",render_triange(tri, camera));
        thread::sleep(Duration::from_millis(100))


    }
}

pub fn genrate_test_world() -> World{
    let camera = Camera{origin:Vec3 { x: 0.0, y: 0.0, z: 0.0 },rotation:Vec3 { x: 0.0, y: 0.0, z: 0.0 },width:90,hight:60,spread:40.0,focal_lenth:50.0};
    let mut world = World::empty_with_defult_materials(camera, 10);
    let mut mesh = Mesh::new_box(vec3(0.0,0.0,-1.0), vec3(1.0,1.0,1.0), vec3(0.0,0.0,0.0));
    let mut mesh2 = Mesh::new_box(vec3(0.0,0.0,-4.0), vec3(1.0,1.0,1.0), vec3(0.0,0.0,0.0));
    mesh.material_id = Some(4);
    mesh2.material_id = Some(3);
    world.meshes = vec![mesh,mesh2];
    world
}
pub fn render_world(world:World) -> (Paragraph<'static>, World){
    let ray_size = (world.camera.width*world.camera.hight) as usize;
    let all_size = ray_size*world.meshes.len();
    let mut world_array :Vec<Vec<i32>> = Vec::with_capacity(all_size);
    for mesh in world.meshes.clone(){
        world_array.push(render_mesh(mesh,world.camera));
    }
    let mut return_array :Vec<i32> = Vec::with_capacity(ray_size);
    for i in 0..ray_size {
        return_array.push(0);
    }
    for array in world_array{
        let mut index = 0;
        for point in array{
            if point != 0{
                return_array[index] = point;
            }
            index +=1;
        }
    } 
    let mut index = 0;
    let atlat = world.mat_atals.clone();
    let at_len= atlat.len();
    let mut line: Vec<Span> = Vec::with_capacity(world.camera.width as usize);
    let mut paragraph:Vec<Line> = Vec::with_capacity(all_size );
    for i in return_array{
        if i>0{
            let mut id  = 1;
            if (i as usize) < at_len{
                id = i as usize;
            }
            let char = atlat[id];
            let mut span = Span::raw("##").bg(char);
            if i == 1{span = Span::raw("##").bg(Color::White)};
            line.push(span);
        }else {
            line.push(Span::raw("..").bg(Color::Black));
        }
        index += 1;
        if index== world.camera.width as usize{
            line.push(Span::raw("\n").bg(Color::Black));
            let write_line = Line::raw("\n");
            paragraph.push(write_line.spans(line));
            line = Vec::with_capacity(world.camera.width as usize);
            index = 0;
        }
    }
    let r_paragraph = Paragraph::new(Text::from(paragraph));
    (r_paragraph,world)
}



