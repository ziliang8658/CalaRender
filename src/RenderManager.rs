use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;

mod Geometry;

use Geometry::Vec3f;


pub struct SceneManager {
    pub models: Vec<Model>,
    pub lights: Vec<Light>,
}


pub struct Model {
    meshId: i32,
    materidId: i32,
    position: Vec3f,
    rotation: Vec3f,
    scaling: Vec3f,
    meshResPath: str,
    matResPath: str,
    albedo_map: str,
    normal_map: str,
    ambient_ligth: str,
    roughness_map: str,
}

pub struct Light {
    lightType: str,
    radius: f32,
    period: u32,
    position: Vec3f,
    Color: Vec3f,

}


impl SceneManager {
    pub fn loadScene(&self, sceneFile: &str) -> bool {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            // Show the line and its number.
            println!("{}. {}", index + 1, line);
        }
        return true;
    }
}

fn parseFromFile(file: &File) {
    let mut reader = BufReader::new(file);
    let mut buf = String::from("");
    let line_index = 0;
    let mut models: Vec<Model> = Vec::new();
    let mut lights: Vec<Model> = Vec::new();
    while (reader.read_line(&mut buf) != 0) {
        if lien_index == 0 {
            if line == 'm' {
                //now we read the model data
                reader.read_line(&mut buf);
                let model_count = buf.trim().parse().expect("it's not a number");
                let mut model_index = 0;
                while model_index < model_count {
                    parseModelInfo(&mut reader, &mut buf, &mut models)
                    model_index += 1;
                }
            }
            if line == 'l' {
                reader.read_line(&mut buf);
                let light_count = buf.trim().parse().expect("it's not a number");
                let mut light_index = 0;
                while light_index < light_count {
                    parseModelInfo(&mut reader, &mut buf, &mut models)
                    model_index += 1;
                }
            }
        }
    }
}


fn parseModelInfo(reader: &mut BufReader<&File>, buf: &mut String, models: &mut Vec<Model>, basePath: &str) -> Model {
    //Firstly, read the meshId and materialId;
    reader.read_line(buf);
    let mut split_info = buf.split(" ");
    if len(split_info) != 2 {}
    let meshId: i32 = split_info.next().unwrap().parse().unwrap();
    let materidId = split_info.next().unwrap().parse().unwrap();
    let meshFilePath = basePath + "/meshes/" + meshId + "_mesh.obj";
    let materialPath = basePath + "/materials/" + materidId + "/" + materidId;
    //Then, read the position info;
    split_info = buf.split(" ");
    let mut modelInfo: Vec<Vec3f> = Vec::new();
    let mut infoIndex = 0;
    while infoIndex < 3 {
        reader.read_line(buf);
        let mut split_info = buf.split(" ");
        modelInfo.push(Vec3f {
            x: split_info.next().unwrap().parse().unwrap(),
            y: split_info.next().unwrap().parse().unwrap(),
            z: split_info.next().unwrap().parse().unwrap(),
        });
        infoIndex += 1;
    }
    loadImageFromMaterial(model, materidId);

    models.push(Model {
        meshId,
        materidId: 0,
        position: Vec3f {
            x: modelInfo.get(0).unwrap().x,
            y: modelInfo.get(0).unwrap().y,
            z: modelInfo.get(0).unwrap().z,
        },
        rotation: Vec3f {
            x: modelInfo.get(1).unwrap().x,
            y: modelInfo.get(1).unwrap().y,
            z: modelInfo.get(1).unwrap().z,
        },
        scaling: Vec3f {
            x: modelInfo.get(2).unwrap().x,
            y: modelInfo.get(2).unwrap().y,
            z: modelInfo.get(2).unwrap().z,
        },
    }
    );

    //Finally, we only need to read an empty line to finish the model parsing process
    reader.read_line(buf);
}


fn parseLightInfo(reader: &mut BufReader<&File>, buf: &mut String, lights: &mut Vec<Light>) -> Model {
    let mut light = Light {
        lightType: "" as str,
        radius: 0.0,
        period: 0,
        position: Vec3f::new(0.0, 0.0, 0.0),
        Color: Vec3f::new(0.0, 0.0, 0.0),
    };
    //Firstly, read the LigthType
    reader.read_line(buf);
    let lightType: &str = buf.trim().clone();
    let mut key = "";
    let mut radius = "";
    let mut period = 0;
    if lightType == "o" || lightType == "l" {
        let mut infoIndex = 0;
        reader.read_line(buf);
        let mut split_info = buf.split(" ");
        key = split_info.next().unwrap().parse().unwrap();
        radius = split_info.next().unwrap().parse().unwrap();
        period = split_info.next().unwrap().parse().unwrap();
    }
    let mut infoIndex = 0;
    while infoIndex < 2 {
        //Then, read the position and Color Info
        split_info = buf.split(" ");
        let mut fieldInfo = 0;
        reader.read_line(buf);
        let mut split_info = buf.split(" ");
        key = split_info.next().unwrap().parse().unwrap();
        if infoIndex == 1 {
            light.position = Vec3f::new(
                x: split_info.next().unwrap().parse().unwrap(),
                y: split_info.next().unwrap().parse().unwrap(),
                z: split_info.next().unwrap().parse().unwrap(),
            )
        } else {
            light.Color = Vec3f::new(
                x: split_info.next().unwrap().parse().unwrap(),
                y: split_info.next().unwrap().parse().unwrap(),
                z: split_info.next().unwrap().parse().unwrap(),
            )
        }
        infoIndex += 1
    }
    //Finally, we only need to read an empty line to finish the model parsing process
    reader.read_line(buf);
    lights.push(light);
}


fn loadImageFromMaterial(model: &mut Model, materialPath: &str) {
    model.albedo_map = materialPath + "_albedo.png";
    model.normal_map = materialPath + "_normal.png";
    model.ambient_ligth = materialPath + "_ao.png";
    model.roughness_map = materialPath + "_rough.png"
}

fn loadResFromMesh(model: &mut Model, meshFilePath: &str) {}